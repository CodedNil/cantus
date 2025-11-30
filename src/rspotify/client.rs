use super::{
    ClientError, ClientResult,
    custom_serde::{duration_second, space_separated_scopes},
    generate_random_string,
    model::{
        Artist, ArtistId, Artists, CurrentPlaybackContext, CurrentUserQueue, Page, Playlist,
        PlaylistId, PlaylistItem, TrackId,
    },
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpListener},
    path::PathBuf,
    sync::Arc,
};
use ureq::Agent;
use url::Url;

const VERIFIER_BYTES: usize = 43;
const REDIRECT_HOST: &str = "127.0.0.1";
const REDIRECT_PORT: u16 = 7474;

/// The [Authorization Code Flow with Proof Key for Code Exchange
/// (PKCE)][reference] client for the Spotify API.
///
/// This flow is very similar to the regular Authorization Code Flow, so please
/// read [`AuthCodeSpotify`](crate::AuthCodeSpotify) for more information about
/// it. The main difference in this case is that you can avoid storing your
/// client secret by generating a *code verifier* and a *code challenge*.
/// However, note that the refresh token obtained with PKCE will only work to
/// request the next one, after which it'll become invalid.
///
/// There's an [example][example-main] available to learn how to use this
/// client.
///
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
/// [example-main]: https://github.com/ramsayleung/rspotify/blob/master/examples/auth_code_pkce.rs
#[derive(Debug)]
pub struct SpotifyClient {
    client_id: String,
    state: String,
    scopes: HashSet<String>,
    cache_path: PathBuf,
    token: Arc<RwLock<Option<Token>>>,
    verifier: String,
    pub http: Agent,
}

/// Spotify access token information
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/authorization/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    /// An access token that can be provided in subsequent calls
    #[serde(rename = "access_token")]
    pub access: String,
    /// The time period for which the access token is valid.
    #[serde(with = "duration_second")]
    pub expires_in: Duration,
    /// The valid time for which the access token is available represented
    /// in ISO 8601 combined date and time.
    pub expires_at: Option<DateTime<Utc>>,
    /// A token that can be sent to the Spotify Accounts service
    /// in place of an authorization code
    #[serde(rename = "refresh_token")]
    pub refresh: Option<String>,
    /// A list of [scopes](https://developer.spotify.com/documentation/general/guides/authorization/scopes/)
    /// which have been granted for this `access_token`
    ///
    /// You may use the `scopes!` macro in
    /// [`rspotify-macros`](https://docs.rs/rspotify-macros) to build it at
    /// compile time easily.
    // The token response from spotify is singular, hence the rename to `scope`
    #[serde(default, with = "space_separated_scopes", rename = "scope")]
    pub scopes: HashSet<String>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            access: String::new(),
            expires_in: Duration::try_seconds(0).unwrap(),
            expires_at: Some(Utc::now()),
            refresh: None,
            scopes: HashSet::new(),
        }
    }
}

impl Token {
    /// Check if the token is expired. It includes a margin of 10 seconds (which
    /// is how much a request would take in the worst case scenario).
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .is_none_or(|expiration| Utc::now() + TimeDelta::try_seconds(10).unwrap() >= expiration)
    }
}

impl SpotifyClient {
    /// Tries to read the cache file's token.
    ///
    /// This will return an error if the token couldn't be read (e.g. it's not
    /// available or the JSON is malformed). It may return `Ok(None)` if:
    ///
    /// * The read token is expired and `allow_expired` is false
    /// * Its scopes don't match with the current client (you will need to
    ///   re-authenticate to gain access to more scopes)
    /// * The cached token is disabled in the config
    ///
    /// # Note
    /// This function's implementation differs slightly from the implementation
    /// in [`ClientCredsSpotify::read_token_cache`]. The boolean parameter
    /// `allow_expired` allows users to load expired tokens from the cache.
    /// This functionality can be used to access the refresh token and obtain
    /// a new, valid token. This option is unavailable in the implementation of
    /// [`ClientCredsSpotify::read_token_cache`] since the client credentials
    /// authorization flow does not have a refresh token and instead requires
    /// the application re-authenticate.
    ///
    /// [`ClientCredsSpotify::read_token_cache`]: crate::client_creds::ClientCredsSpotify::read_token_cache
    fn read_token_cache(&self, allow_expired: bool) -> ClientResult<Option<Token>> {
        let token: Token = serde_json::from_str(&fs::read_to_string(&self.cache_path)?)?;
        if !self.scopes.is_subset(&token.scopes) || (!allow_expired && token.is_expired()) {
            // Invalid token, since it doesn't have at least the currently required scopes or it's expired.
            Ok(None)
        } else {
            Ok(Some(token))
        }
    }

    /// Parse the response code in the given response url. If the URL cannot be
    /// parsed or the `code` parameter is not present, this will return `None`.
    ///
    // As the [RFC
    // indicates](https://datatracker.ietf.org/doc/html/rfc6749#section-4.1),
    // the state should be the same between the request and the callback. This
    // will also return `None` if this is not true.
    fn parse_response_code(&self, url: &str) -> Option<String> {
        let mut code = None;
        let mut state = None;
        for (key, value) in Url::parse(url).ok()?.query_pairs() {
            if key == "code" {
                code = Some(value.to_string());
            } else if key == "state" {
                state = Some(value.to_string());
            }
        }

        // Making sure the state is the same
        if state.as_deref() != Some(&self.state) {
            tracing::error!("Request state doesn't match the callback state");
            return None;
        }
        code
    }

    /// Spawn HTTP server at provided socket address to accept OAuth callback and return auth code.
    fn get_authcode_listener(&self, socket_address: SocketAddr) -> String {
        let listener = TcpListener::bind(socket_address).unwrap();

        // The server will terminate itself after collecting the first code.
        let mut stream = listener.incoming().flatten().next().unwrap();
        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let redirect_url = request_line.split_whitespace().nth(1).unwrap();
        let redirect_full_url =
            format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback{redirect_url}");

        let code = self.parse_response_code(&redirect_full_url).unwrap();

        let message = "Go back to your terminal :)";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes()).unwrap();

        code
    }

    /// Opens up the authorization URL in the user's browser so that it can
    /// authenticate. It reads from the standard input the redirect URI
    /// in order to obtain the access token information. The resulting access
    /// token will be saved internally once the operation is successful.
    pub fn prompt_for_token(&self, url: &str) -> Token {
        if let Ok(Some(new_token)) = self.read_token_cache(true) {
            if new_token.is_expired() {
                // Ensure that we actually got a token from the refetch
                if let Some(refreshed_token) = self.refetch_token().unwrap() {
                    return refreshed_token;
                }
            } else {
                return new_token;
            }
        }
        match webbrowser::open(url) {
            Ok(()) => println!("Opened {url} in your browser."),
            Err(why) => eprintln!(
                "Error when trying to open an URL in your browser: {why:?}. \
                 Please navigate here manually: {url}"
            ),
        }
        let code = self.get_authcode_listener(SocketAddr::new(
            REDIRECT_HOST.parse::<IpAddr>().unwrap(),
            REDIRECT_PORT,
        ));
        self.fetch_access_token(&[
            ("grant_type", "authorization_code"),
            ("code", &code),
            (
                "redirect_uri",
                &format!("http://{REDIRECT_HOST}:{REDIRECT_PORT}/callback"),
            ),
            ("client_id", &self.client_id),
            ("code_verifier", &self.verifier),
        ])
        .unwrap()
    }

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
        self.api_post(
            &format!("playlists/{playlist_id}/tracks"),
            &json!({
                "uris": [format!("spotify:track:{track_id}")],
            }),
        )
    }

    /// Removes all occurrences of the given items from the given playlist.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-playlist)
    pub fn playlist_remove_item(
        &self,
        playlist_id: &PlaylistId,
        track_id: &TrackId,
    ) -> ClientResult<()> {
        self.api_delete(
            &format!("playlists/{playlist_id}/tracks"),
            &json!({
                "tracks": [{
                    "uri": format!("spotify:track:{track_id}")
                }],
            }),
        )
    }

    /// Remove one or more tracks from the users liked songs.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user)
    pub fn current_user_saved_tracks_delete(&self, track_id: &TrackId) -> ClientResult<()> {
        self.api_delete(&format!("me/tracks/?ids={track_id}"), &Value::Null)?;
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
        self.api_put(&format!("me/tracks/?ids={track_id}"), &Value::Null)?;
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
        self.api_put("me/player/pause", &Value::Null)
    }

    /// Resume a User’s Playback.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback)
    pub fn resume_playback(&self) -> ClientResult<()> {
        self.api_put("me/player/play", &Value::Null)
    }

    /// Skip User’s Playback To Next Track.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-next-track)
    pub fn next_track(&self) -> ClientResult<()> {
        self.api_post("me/player/next", &Value::Null)
    }

    /// Skip User’s Playback To Previous Track.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-previous-track)
    pub fn previous_track(&self) -> ClientResult<()> {
        self.api_post("me/player/previous", &Value::Null)
    }

    /// Seek To Position In Currently Playing Track.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/seek-to-position-in-currently-playing-track)
    pub fn seek_track(&self, position: Duration) -> ClientResult<()> {
        self.api_put(
            &format!("me/player/seek?position_ms={}", position.num_milliseconds()),
            &Value::Null,
        )
    }

    /// Set Volume For User’s Playback.
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/set-volume-for-users-playback)
    pub fn volume(&self, volume_percent: u8) -> ClientResult<()> {
        debug_assert!(
            volume_percent <= 100u8,
            "volume must be between 0 and 100, inclusive"
        );
        self.api_put(
            &format!("me/player/volume?volume_percent={volume_percent}"),
            &Value::Null,
        )
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
        if self.token.read().as_ref().is_some_and(Token::is_expired) {
            *self.token.write() = self.refetch_token()?;
            self.write_token_cache();
        }
        Ok(format!(
            "Bearer {}",
            self.token
                .read()
                .as_ref()
                .ok_or(ClientError::InvalidToken)?
                .access
        ))
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
    fn api_post(&self, url: &str, payload: &Value) -> ClientResult<()> {
        self.http
            .post(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .send_json(payload)?;
        Ok(())
    }

    /// Convenience method to send PUT requests related to an endpoint in the API.
    fn api_put(&self, url: &str, payload: &Value) -> ClientResult<()> {
        self.http
            .put(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .send_json(payload)?;
        Ok(())
    }

    /// Convenience method to send DELETE requests related to an endpoint in the API.
    fn api_delete(&self, url: &str, payload: &Value) -> ClientResult<()> {
        self.http
            .delete(format!("https://api.spotify.com/v1/{url}"))
            .header("authorization", self.auth_headers()?)
            .force_send_body()
            .send_json(payload)?;
        Ok(())
    }

    /// Updates the cache file at the internal cache path.
    fn write_token_cache(&self) {
        if let Some(token) = self.token.read().as_ref() {
            fs::write(&self.cache_path, serde_json::to_string(&token).unwrap()).unwrap();
        }
    }

    /// Sends a request to Spotify for an access token.
    fn fetch_access_token(&self, payload: &[(&str, &str)]) -> ClientResult<Token> {
        let response = self
            .http
            .post("https://accounts.spotify.com/api/token".to_owned())
            .send_form(payload.to_owned())?
            .into_body()
            .read_to_string()?;
        let mut token = serde_json::from_str::<Token>(&response)?;
        token.expires_at = Utc::now().checked_add_signed(token.expires_in);
        Ok(token)
    }

    fn refetch_token(&self) -> ClientResult<Option<Token>> {
        match self.token.read().as_ref() {
            Some(Token {
                refresh: Some(refresh_token),
                ..
            }) => {
                let token = self.fetch_access_token(&[
                    ("grant_type", "refresh_token"),
                    ("refresh_token", refresh_token),
                    ("client_id", &self.client_id),
                ])?;

                Ok(Some(token))
            }
            _ => Ok(None),
        }
    }

    /// Same as [`Self::new`] but with an extra parameter to configure the client.
    pub fn new(client_id: String, scopes: HashSet<String>, cache_path: PathBuf) -> Self {
        let state = generate_random_string(
            16,
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        );
        let (verifier, url) = get_authorize_url(&client_id, &scopes, &state).unwrap();
        let spotify_client = Self {
            client_id,
            state,
            scopes,
            cache_path,
            token: Arc::new(RwLock::new(None)),
            verifier,
            http: Agent::new_with_defaults(),
        };
        *spotify_client.token.write() = Some(spotify_client.prompt_for_token(&url));
        spotify_client.write_token_cache();
        spotify_client
    }
}

/// Returns the URL needed to authorize the current client as the first step in the authorization flow.
///
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
/// [rfce]: https://datatracker.ietf.org/doc/html/rfc7636#section-4.1
pub fn get_authorize_url(
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
