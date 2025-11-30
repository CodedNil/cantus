use arrayvec::ArrayString;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpListener},
    path::PathBuf,
};
use thiserror::Error;
use ureq::Agent;
use url::Url;

const VERIFIER_BYTES: usize = 43;
const REDIRECT_HOST: &str = "127.0.0.1";
const REDIRECT_PORT: u16 = 7474;

/// The [Authorization Code Flow with Proof Key for Code Exchange
/// (PKCE)][reference] client for the Spotify API.
///
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
#[derive(Debug)]
pub struct SpotifyClient {
    client_id: String,
    cache_path: PathBuf,
    token: RwLock<Token>,
    pub http: Agent,
}

// Albums
pub type AlbumId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Album {
    pub id: AlbumId,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    pub image: String,
}

// Artists
pub type ArtistId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    pub image: String,
}

#[derive(Deserialize)]
pub struct Artists {
    pub artists: Vec<Artist>,
}

// Track
pub type TrackId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub album: Album,
    #[serde(deserialize_with = "deserialize_first_artist", rename = "artists")]
    pub artist: Artist,
    pub duration_ms: u32,
}

#[derive(Deserialize)]
pub struct PartialTrack {
    pub id: TrackId,
}

// Playlist
pub type PlaylistId = ArrayString<22>;

/// Simplified playlist object
#[derive(Deserialize)]
pub struct Playlist {
    pub id: PlaylistId,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_images", rename = "images")]
    pub image: String,
    pub snapshot_id: ArrayString<32>,
    #[serde(deserialize_with = "deserialize_tracks_total", rename = "tracks")]
    pub total_tracks: u32,
}

/// Playlist track object
#[derive(Deserialize)]
pub struct PlaylistItem {
    pub track: PartialTrack,
}

/// Context object
#[derive(Deserialize)]
pub struct Context {
    /// The URI may be of any type, so it's not parsed into a [`crate::Id`]
    pub uri: String,
}

#[derive(Deserialize)]
pub struct CurrentPlaybackContext {
    pub device: Device,
    pub context: Option<Context>,
    #[serde(default)]
    pub progress_ms: u32,
    pub is_playing: bool,
    pub item: Option<Track>,
}

#[derive(Deserialize)]
pub struct CurrentUserQueue {
    pub currently_playing: Option<Track>,
    pub queue: Vec<Track>,
}

/// Device object
#[derive(Deserialize)]
pub struct Device {
    pub volume_percent: Option<u32>,
}

/// Spotify access token information
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/authorization/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Token {
    /// An access token that can be provided in subsequent calls
    #[serde(rename = "access_token")]
    access: ArrayString<359>,
    /// Number of seconds for which the access token is valid.
    expires_in: u32,
    /// The valid time for which the access token is available represented in ISO 8601 combined date and time.
    expires_at: Option<DateTime<Utc>>,
    /// A token that can be sent to the Spotify Accounts service in place of an authorization code
    #[serde(rename = "refresh_token")]
    refresh: Option<ArrayString<131>>,
    /// A list of [scopes](https://developer.spotify.com/documentation/general/guides/authorization/scopes/) which have been granted for this `access_token`
    #[serde(default, with = "space_separated_scopes", rename = "scope")]
    scopes: HashSet<String>,
}

impl Token {
    /// Check if the token is expired. It includes a margin of 10 seconds (which is how much a request would take in the worst case scenario).
    fn is_expired(&self) -> bool {
        self.expires_at
            .is_none_or(|expiration| Utc::now() + TimeDelta::try_seconds(10).unwrap() >= expiration)
    }
}

/// Tries to read the cache file's token.
///
/// This will return an error if the token couldn't be read (e.g. it's not
/// available or the JSON is malformed). It may return `Ok(None)` if:
///
/// * The read token is expired and `allow_expired` is false
/// * Its scopes don't match with the current client (you will need to
///   re-authenticate to gain access to more scopes)
/// * The cached token is disabled in the config
fn read_token_cache(
    allow_expired: bool,
    cache_path: &PathBuf,
    scopes: &HashSet<String>,
) -> Result<Option<Token>, std::io::Error> {
    let token: Token = serde_json::from_str(&fs::read_to_string(cache_path)?)?;
    if !scopes.is_subset(&token.scopes) || (!allow_expired && token.is_expired()) {
        // Invalid token, since it doesn't have at least the currently required scopes or it's expired.
        Ok(None)
    } else {
        Ok(Some(token))
    }
}

/// Opens up the authorization URL in the user's browser so that it can
/// authenticate. It reads from the standard input the redirect URI
/// in order to obtain the access token information. The resulting access
/// token will be saved internally once the operation is successful.
fn prompt_for_token(
    url: &str,
    cache_path: &PathBuf,
    scopes: &HashSet<String>,
    client_id: &str,
    verifier: &str,
    http: &Agent,
) -> Token {
    if let Ok(Some(new_token)) = read_token_cache(true, cache_path, scopes)
        && !new_token.is_expired()
    {
        return new_token;
    }
    match webbrowser::open(url) {
        Ok(()) => println!("Opened {url} in your browser."),
        Err(why) => eprintln!(
            "Error when trying to open an URL in your browser: {why:?}. \
             Please navigate here manually: {url}"
        ),
    }

    // Start a server to listen for the callback
    let listener = TcpListener::bind(SocketAddr::new(
        REDIRECT_HOST.parse::<IpAddr>().unwrap(),
        REDIRECT_PORT,
    ))
    .unwrap();

    // The server will terminate itself after collecting the first code.
    let mut stream = listener.incoming().flatten().next().unwrap();
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    let redirect_full_url =
        format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback{redirect_url}");

    let mut code = None;
    for (key, value) in Url::parse(&redirect_full_url).ok().unwrap().query_pairs() {
        if key == "code" {
            code = Some(value.to_string());
        }
    }
    let code = code.unwrap();

    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).unwrap();

    // Get token from spotify
    let payload: &[(&str, &str)] = &[
        ("grant_type", "authorization_code"),
        ("code", &code),
        (
            "redirect_uri",
            &format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback"),
        ),
        ("client_id", client_id),
        ("code_verifier", verifier),
    ];
    let response = http
        .post("https://accounts.spotify.com/api/token".to_owned())
        .send_form(payload.to_owned())
        .unwrap()
        .into_body()
        .read_to_string()
        .unwrap();
    let mut token = serde_json::from_str::<Token>(&response).unwrap();
    token.expires_at =
        Utc::now().checked_add_signed(Duration::seconds(i64::from(token.expires_in)));
    token
}

impl SpotifyClient {
    /// Get current user playlists without required getting his profile.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-list-of-current-users-playlists)
    pub fn current_user_playlists(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<Playlist>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = [("limit", limit.as_deref()), ("offset", offset.as_deref())];

        let result = self.api_get("me/playlists", &params)?;
        serde_json::from_str(&result).map_err(Into::into)
    }

    /// Adds items to a playlist.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/add-tracks-to-playlist)
    pub fn playlist_add_item(
        &self,
        playlist_id: &PlaylistId,
        track_id: &TrackId,
    ) -> ClientResult<()> {
        let track_uri = format!("spotify:track:{track_id}");
        self.api_post_payload(
            &format!("playlists/{playlist_id}/tracks"),
            &format!(r#"{{"uris": ["{track_uri}"]}}"#),
        )
    }

    /// Removes all occurrences of the given items from the given playlist.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-playlist)
    pub fn playlist_remove_item(
        &self,
        playlist_id: &PlaylistId,
        track_id: &TrackId,
    ) -> ClientResult<()> {
        let track_uri = format!("spotify:track:{track_id}");
        self.api_delete_payload(
            &format!("playlists/{playlist_id}/tracks"),
            &format!(r#"{{"tracks": [ {{"uri": "{track_uri}"}} ]}}"#),
        )
    }

    /// Remove one or more tracks from the users liked songs.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user)
    pub fn current_user_saved_tracks_delete(&self, track_id: &TrackId) -> ClientResult<()> {
        self.api_delete(&format!("me/tracks/?ids={track_id}"))?;
        Ok(())
    }

    /// Check if one or more tracks is already liked by the user.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-tracks)
    pub fn current_user_saved_tracks_contains(
        &self,
        track_id: &TrackId,
    ) -> ClientResult<Vec<bool>> {
        serde_json::from_str(&self.api_get(&format!("me/tracks/contains/?ids={track_id}"), &[])?)
            .map_err(Into::into)
    }

    /// Save one or more tracks to the users liked songs.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user)
    pub fn current_user_saved_tracks_add(&self, track_id: &TrackId) -> ClientResult<()> {
        self.api_put(&format!("me/tracks/?ids={track_id}"))?;
        Ok(())
    }

    /// Get Information About The User’s Current Playback
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-information-about-the-users-current-playback)
    pub fn current_playback(&self) -> ClientResult<Option<CurrentPlaybackContext>> {
        let result = self.api_get("me/player", &[])?;
        if result.is_empty() {
            Ok(None)
        } else {
            serde_json::from_str(&result).map_err(Into::into)
        }
    }

    /// Get the Current User’s Queue
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-queue)
    pub fn current_user_queue(&self) -> ClientResult<CurrentUserQueue> {
        let result = self.api_get("me/player/queue", &[])?;
        serde_json::from_str(&result).map_err(Into::into)
    }

    /// Pause a User’s Playback.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/pause-a-users-playback)
    pub fn pause_playback(&self) -> ClientResult<()> {
        self.api_put("me/player/pause")
    }

    /// Resume a User’s Playback.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback)
    pub fn resume_playback(&self) -> ClientResult<()> {
        self.api_put("me/player/play")
    }

    /// Skip User’s Playback To Next Track.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-next-track)
    pub fn next_track(&self) -> ClientResult<()> {
        self.api_post("me/player/next")
    }

    /// Skip User’s Playback To Previous Track.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-previous-track)
    pub fn previous_track(&self) -> ClientResult<()> {
        self.api_post("me/player/previous")
    }

    /// Seek To Position In Currently Playing Track.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/seek-to-position-in-currently-playing-track)
    pub fn seek_track(&self, position: Duration) -> ClientResult<()> {
        self.api_put(&format!(
            "me/player/seek?position_ms={}",
            position.num_milliseconds()
        ))
    }

    /// Set Volume For User’s Playback.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/set-volume-for-users-playback)
    pub fn volume(&self, volume_percent: u8) -> ClientResult<()> {
        debug_assert!(
            volume_percent <= 100u8,
            "volume must be between 0 and 100, inclusive"
        );
        self.api_put(&format!("me/player/volume?volume_percent={volume_percent}"))
    }

    /// Returns a list of artists given the artist IDs, URIs, or URLs.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-artists)
    pub fn artists(&self, artist_ids: &[ArtistId]) -> ClientResult<Vec<Artist>> {
        serde_json::from_str::<Artists>(
            &self.api_get(&format!("artists/?ids={}", artist_ids.join(",")), &[])?,
        )
        .map_err(Into::into)
        .map(|x| x.artists)
    }

    /// Get full details of the items of a playlist owned by a user.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlists-tracks)
    pub fn playlist_items(
        &self,
        playlist_id: &PlaylistId,
        fields: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<PlaylistItem>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        serde_json::from_str(&self.api_get(
            &format!("playlists/{playlist_id}/tracks"),
            &[
                ("fields", fields),
                ("limit", limit.as_deref()),
                ("offset", offset.as_deref()),
            ],
        )?)
        .map_err(Into::into)
    }

    /// The headers required for authenticated requests to the API.
    ///
    /// Since this is accessed by authenticated requests always, it's where the
    /// automatic reauthentication takes place, if enabled.
    fn auth_headers(&self) -> ClientResult<String> {
        if self.token.read().is_expired() {
            if let Ok(token) = self.refetch_token() {
                *self.token.write() = token;
                self.write_token_cache();
            } else {
                return Err(ClientError::InvalidToken);
            }
        }
        Ok(format!("Bearer {}", self.token.read().access))
    }

    // HTTP-related methods for the Spotify client. They wrap up the basic HTTP
    // client with its specific usage for endpoints or authentication.

    /// Convenience method to send GET requests related to an endpoint in the  API.
    fn api_get(&self, url: &str, payload: &[(&str, Option<&str>)]) -> ClientResult<String> {
        let response = self
            .http
            .get(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .query_pairs(
                payload
                    .iter()
                    .filter_map(|&(k, v)| v.map(|v_inner| (k, v_inner))),
            )
            .call()?;
        Ok(response.into_body().read_to_string()?)
    }

    /// Convenience method to send POST requests related to an endpoint in the API.
    fn api_post(&self, url: &str) -> ClientResult<()> {
        self.http
            .post(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .send_empty()?;
        Ok(())
    }

    /// Convenience method to send POST requests related to an endpoint in the API.
    fn api_post_payload(&self, url: &str, payload: &str) -> ClientResult<()> {
        self.http
            .post(format!("https://api.spotify.com/v1/{url}"))
            .header("Content-Type", "application/json; charset=utf-8")
            .header("authorization", self.auth_headers()?)
            .send(payload)?;
        Ok(())
    }

    /// Convenience method to send PUT requests related to an endpoint in the API.
    fn api_put(&self, url: &str) -> ClientResult<()> {
        self.http
            .put(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .send_empty()?;
        Ok(())
    }

    /// Convenience method to send DELETE requests related to an endpoint in the API.
    fn api_delete(&self, url: &str) -> ClientResult<()> {
        self.http
            .delete(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .call()?;
        Ok(())
    }

    /// Convenience method to send DELETE requests related to an endpoint in the API.
    fn api_delete_payload(&self, url: &str, payload: &str) -> ClientResult<()> {
        self.http
            .delete(format!("https://api.spotify.com/v1/{url}"))
            .header("Content-Type", "application/json; charset=utf-8")
            .header("authorization", self.auth_headers()?)
            .force_send_body()
            .send(payload)?;
        Ok(())
    }

    /// Updates the cache file at the internal cache path.
    fn write_token_cache(&self) {
        let token = self.token.read().clone();
        fs::write(&self.cache_path, serde_json::to_string(&token).unwrap()).unwrap();
    }

    /// Sends a request to Spotify for a new token.
    fn refetch_token(&self) -> ClientResult<Token> {
        let Some(refresh_token) = &self.token.read().refresh else {
            return Err(ClientError::InvalidToken);
        };
        let payload: &[(&str, &str)] = &[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &self.client_id),
        ];
        let response = self
            .http
            .post("https://accounts.spotify.com/api/token".to_owned())
            .send_form(payload.to_owned())?
            .into_body()
            .read_to_string()?;
        let mut token = serde_json::from_str::<Token>(&response)?;
        token.expires_at =
            Utc::now().checked_add_signed(Duration::seconds(i64::from(token.expires_in)));
        Ok(token)
    }

    /// Same as [`Self::new`] but with an extra parameter to configure the client.
    pub fn new(client_id: String, scopes: &HashSet<String>, cache_path: PathBuf) -> Self {
        let state = generate_random_string(
            16,
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        );
        let (verifier, url) = get_authorize_url(&client_id, scopes, &state).unwrap();
        let agent = Agent::new_with_defaults();
        let token = prompt_for_token(&url, &cache_path, scopes, &client_id, &verifier, &agent);
        let spotify_client = Self {
            client_id,
            cache_path,
            token: RwLock::new(token),
            http: Agent::new_with_defaults(),
        };
        spotify_client.write_token_cache();
        spotify_client
    }
}

/// Returns the URL needed to authorize the current client as the first step in the authorization flow.
///
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
/// [rfce]: https://datatracker.ietf.org/doc/html/rfc7636#section-4.1
fn get_authorize_url(
    client_id: &str,
    scopes: &HashSet<String>,
    state: &str,
) -> ClientResult<(String, String)> {
    let verifier = generate_random_string(
        VERIFIER_BYTES,
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~",
    );

    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());

    let parsed = Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        &[
            ("client_id", client_id),
            ("response_type", "code"),
            (
                "redirect_uri",
                &format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback"),
            ),
            ("code_challenge_method", "S256"),
            ("code_challenge", &challenge),
            ("state", state),
            (
                "scope",
                scopes
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join(" ")
                    .as_str(),
            ),
        ],
    )?;
    Ok((verifier, parsed.into()))
}

/// Generate `length` random chars
fn generate_random_string(length: usize, alphabet: &[u8]) -> String {
    let range = alphabet.len();
    (0..length)
        .map(|_| alphabet[fastrand::usize(..range)] as char)
        .collect()
}

/// Possible errors returned from the `rspotify` client.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),

    #[error("http error: {0}")]
    Http(String),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Token is not valid")]
    InvalidToken,
}

// The conversion has to be done manually because it's in a `Box<T>`
impl From<ureq::Error> for ClientError {
    fn from(err: ureq::Error) -> Self {
        Self::Http(err.to_string())
    }
}

pub type ClientResult<T> = Result<T, ClientError>;

mod space_separated_scopes {
    use serde::{Deserialize, Serializer, de};
    use std::collections::HashSet;

    pub fn deserialize<'de, D>(d: D) -> Result<HashSet<String>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let scopes: String = Deserialize::deserialize(d)?;
        Ok(scopes.split_whitespace().map(ToOwned::to_owned).collect())
    }

    pub fn serialize<S>(scopes: &HashSet<String>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let scopes = scopes.clone().into_iter().collect::<Vec<_>>().join(" ");
        s.serialize_str(&scopes)
    }
}

#[derive(Deserialize)]
struct TracksRef {
    total: u32,
}

fn deserialize_tracks_total<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let tracks_ref = TracksRef::deserialize(deserializer)?;
    Ok(tracks_ref.total)
}

/// Expects an array of Image structs and returns the URL of the image with the minimum width, wrapped in an Option.
#[derive(Deserialize)]
struct Image {
    url: String,
    width: Option<u32>,
}
fn deserialize_images<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let images: Vec<Image> = Vec::deserialize(deserializer)?;
    Ok(images
        .into_iter()
        .min_by_key(|img| img.width)
        .map(|img| img.url)
        .unwrap())
}

/// Expects the first item from an array.
fn deserialize_first_artist<'de, D>(deserializer: D) -> Result<Artist, D::Error>
where
    D: Deserializer<'de>,
{
    let artists: Vec<Artist> = Vec::deserialize(deserializer)?;
    Ok(artists.into_iter().next().unwrap())
}

/// Custom deserializer to handle `Vec<Option<T>>` and filter out `None` values
/// This is useful for deserializing lists that may contain null values that are not relevants
fn vec_without_nulls<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let v = Vec::<Option<T>>::deserialize(deserializer)?;
    Ok(v.into_iter().flatten().collect())
}

#[derive(Deserialize)]
pub struct Page<T: DeserializeOwned> {
    #[serde(deserialize_with = "vec_without_nulls")]
    pub items: Vec<T>,
    pub total: u32,
}
