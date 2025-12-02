use arrayvec::ArrayString;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use parking_lot::RwLock;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpListener},
    path::PathBuf,
};
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use ureq::Agent;
use url::Url;

const VERIFIER_BYTES: usize = 43;
const REDIRECT_HOST: &str = "127.0.0.1";
const REDIRECT_PORT: u16 = 7474;

/// The [Authorization Code Flow with Proof Key for Code Exchange (PKCE)][reference] client for the Spotify API.
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Token {
    /// An access token that can be provided in subsequent calls
    #[serde(rename = "access_token")]
    access: String,
    /// Number of seconds for which the access token is valid.
    expires_in: u32,
    /// The valid time for which the access token is available represented in ISO 8601 combined date and time.
    expires_at: Option<OffsetDateTime>,
    /// A token that can be sent to the Spotify Accounts service in place of an authorization code
    #[serde(rename = "refresh_token")]
    refresh: Option<String>,
    /// A list of [scopes](https://developer.spotify.com/documentation/general/guides/authorization/scopes/) which have been granted for this `access_token`
    #[serde(
        serialize_with = "serialize_scopes",
        deserialize_with = "deserialize_scopes",
        rename = "scope"
    )]
    scopes: HashSet<String>,
}

impl Token {
    /// Check if the token is expired. It includes a margin of 10 seconds (which is how much a request would take in the worst case scenario).
    fn is_expired(&self) -> bool {
        self.expires_at.is_none_or(|expiration| {
            OffsetDateTime::now_utc() + Duration::seconds(10) >= expiration
        })
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
    if let Ok(Some(cached)) = read_token_cache(true, cache_path, scopes) {
        return cached;
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
    let request_path = request_line.split_whitespace().nth(1).unwrap();
    let redirect_full_url =
        format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback{request_path}");
    let code = Url::parse(&redirect_full_url)
        .unwrap()
        .query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.into_owned())
        .unwrap();
    let message = "Cantus connected successfully, this tab can be closed.";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).unwrap();

    let response = http
        .post("https://accounts.spotify.com/api/token")
        .send_form([
            ("grant_type", "authorization_code"),
            ("code", &code),
            (
                "redirect_uri",
                &format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback"),
            ),
            ("client_id", client_id),
            ("code_verifier", verifier),
        ])
        .unwrap()
        .into_body()
        .read_to_string()
        .unwrap();
    let mut token = serde_json::from_str::<Token>(&response).unwrap();
    token.expires_at =
        OffsetDateTime::now_utc().checked_add(Duration::seconds(i64::from(token.expires_in)));
    token
}

impl SpotifyClient {
    /// The headers required for authenticated requests to the API.
    ///
    /// Since this is accessed by authenticated requests always, it's where the
    /// automatic reauthentication takes place, if enabled.
    fn auth_headers(&self) -> ClientResult<String> {
        if self.token.read().is_expired() {
            let token = self.refetch_token()?;
            *self.token.write() = token;
            self.write_token_cache();
        }
        Ok(format!("Bearer {}", self.token.read().access))
    }

    /// Convenience method to send GET requests related to an endpoint in the API.
    pub fn api_get(&self, url: &str) -> ClientResult<String> {
        let response = self
            .http
            .get(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .call()?;
        Ok(response.into_body().read_to_string()?)
    }

    /// Convenience method to send GET requests related to an endpoint in the API.
    pub fn api_get_payload(&self, url: &str, payload: &[(&str, &str)]) -> ClientResult<String> {
        let response = self
            .http
            .get(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .query_pairs(payload.iter().copied())
            .call()?;
        Ok(response.into_body().read_to_string()?)
    }

    /// Convenience method to send POST requests related to an endpoint in the API.
    pub fn api_post(&self, url: &str) -> ClientResult<()> {
        self.http
            .post(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .send_empty()?;
        Ok(())
    }

    /// Convenience method to send POST requests related to an endpoint in the API.
    pub fn api_post_payload(&self, url: &str, payload: &str) -> ClientResult<()> {
        self.http
            .post(format!("https://api.spotify.com/v1/{url}"))
            .header("Content-Type", "application/json; charset=utf-8")
            .header("authorization", self.auth_headers()?)
            .send(payload)?;
        Ok(())
    }

    /// Convenience method to send PUT requests related to an endpoint in the API.
    pub fn api_put(&self, url: &str) -> ClientResult<()> {
        self.http
            .put(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .send_empty()?;
        Ok(())
    }

    /// Convenience method to send DELETE requests related to an endpoint in the API.
    pub fn api_delete(&self, url: &str) -> ClientResult<()> {
        self.http
            .delete(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .call()?;
        Ok(())
    }

    /// Convenience method to send DELETE requests related to an endpoint in the API.
    pub fn api_delete_payload(&self, url: &str, payload: &str) -> ClientResult<()> {
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
        fs::write(
            &self.cache_path,
            serde_json::to_string(&*self.token.read()).unwrap(),
        )
        .unwrap();
    }

    /// Sends a request to Spotify for a new token.
    fn refetch_token(&self) -> ClientResult<Token> {
        let Some(refresh_token) = &self.token.read().refresh else {
            return Err(ClientError::InvalidToken);
        };
        let response = self
            .http
            .post("https://accounts.spotify.com/api/token")
            .send_form([
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", &self.client_id),
            ])?
            .into_body()
            .read_to_string()?;
        let mut token = serde_json::from_str::<Token>(&response)?;
        token.expires_at =
            OffsetDateTime::now_utc().checked_add(Duration::seconds(i64::from(token.expires_in)));
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

impl From<ureq::Error> for ClientError {
    fn from(err: ureq::Error) -> Self {
        Self::Http(err.to_string())
    }
}

pub type ClientResult<T> = Result<T, ClientError>;

fn deserialize_scopes<'de, D>(d: D) -> Result<HashSet<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let scopes: String = Deserialize::deserialize(d)?;
    Ok(scopes.split_whitespace().map(ToOwned::to_owned).collect())
}

fn serialize_scopes<S>(scopes: &HashSet<String>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let scopes = scopes.clone().into_iter().collect::<Vec<_>>().join(" ");
    s.serialize_str(&scopes)
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
