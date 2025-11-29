use crate::rspotify::{
    ClientError, ClientResult, Config, Credentials, OAuth, Token, alphabets,
    generate_random_string,
    model::{
        Artist, ArtistId, Artists, CurrentPlaybackContext, CurrentUserQueue, Page, Playlist,
        PlaylistId, PlaylistItem, TrackId,
    },
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::Utc;
use parking_lot::RwLock;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener},
    sync::Arc,
};
use tracing::error;
use ureq::Agent;
use url::Url;

const VERIFIER_BYTES: usize = 43;

/// Append device ID to an API path.
fn append_device_id(path: &str, device_id: Option<&str>) -> String {
    let mut new_path = path.to_owned();
    if let Some(device_id) = device_id {
        if path.contains('?') {
            let _ = write!(new_path, "&device_id={device_id}");
        } else {
            let _ = write!(new_path, "?device_id={device_id}");
        }
    }
    new_path
}

/// Returns the absolute URL for an endpoint in the API.
pub fn api_url(url: &str) -> String {
    format!("https://api.spotify.com/v1/{url}")
}

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
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Arc<RwLock<Option<Token>>>,
    pub verifier: Option<String>,
    pub http: Agent,
}

impl Default for SpotifyClient {
    fn default() -> Self {
        Self {
            creds: Credentials::default(),
            oauth: OAuth::default(),
            config: Config::default(),
            token: Arc::new(RwLock::new(None)),
            verifier: None,
            http: Agent::new_with_defaults(),
        }
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
    pub fn read_token_cache(&self, allow_expired: bool) -> ClientResult<Option<Token>> {
        let token = Token::from_cache(&self.config.cache_path)?;
        if !self.oauth.scopes.is_subset(&token.scopes) || (!allow_expired && token.is_expired()) {
            // Invalid token, since it doesn't have at least the currently
            // required scopes or it's expired.
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
    pub fn parse_response_code(&self, url: &str) -> Option<String> {
        let url = Url::parse(url).ok()?;
        let params = url.query_pairs().collect::<HashMap<_, _>>();

        let code = params.get("code")?;

        // Making sure the state is the same
        let expected_state = &self.oauth.state;
        let state = params.get("state").map(AsRef::as_ref);
        if state != Some(expected_state) {
            tracing::error!("Request state doesn't match the callback state");
            return None;
        }

        Some(code.to_string())
    }

    /// Spawn HTTP server at provided socket address to accept OAuth callback and return auth code.
    pub fn get_authcode_listener(&self, socket_address: SocketAddr) -> ClientResult<String> {
        let listener =
            TcpListener::bind(socket_address).map_err(|e| ClientError::AuthCodeListenerBind {
                addr: socket_address,
                e,
            })?;

        // The server will terminate itself after collecting the first code.
        let mut stream = listener
            .incoming()
            .flatten()
            .next()
            .ok_or(ClientError::AuthCodeListenerTerminated)?;
        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader
            .read_line(&mut request_line)
            .map_err(|_| ClientError::AuthCodeListenerRead)?;

        let redirect_url = request_line
            .split_whitespace()
            .nth(1)
            .ok_or(ClientError::AuthCodeListenerRead)?;
        let redirect_full_url = format!("{}{}", self.oauth.redirect_uri, redirect_url);

        let code = self
            .parse_response_code(&redirect_full_url)
            .ok_or_else(|| ClientError::AuthCodeListenerParse(redirect_full_url))?;

        let message = "Go back to your terminal :)";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream
            .write_all(response.as_bytes())
            .map_err(|_| ClientError::AuthCodeListenerWrite)?;

        Ok(code)
    }

    // If the specified `redirect_url` is HTTP, loopback, and contains a port,
    // then the corresponding socket address is returned.
    pub fn get_socket_address(&self) -> Option<SocketAddr> {
        let (host, port) = {
            let parsed_url = Url::parse(&self.oauth.redirect_uri).ok()?;
            let port = match parsed_url.scheme() {
                "http" => parsed_url.port().unwrap_or(80),
                "https" => parsed_url.port().unwrap_or(443),
                _ => return None,
            };
            (String::from(parsed_url.host_str()?), port)
        };

        // Handle IPv6 addresses (they come with brackets from host_str())
        let (ip_addr, fallback_ip) = if host.starts_with('[') && host.ends_with(']') {
            // Remove the brackets for IPv6 address parsing
            let ip_str = &host[1..host.len() - 1];
            (ip_str.parse::<IpAddr>(), IpAddr::V6(Ipv6Addr::UNSPECIFIED))
        } else {
            // Regular IPv4 address
            (host.parse::<IpAddr>(), IpAddr::V4(Ipv4Addr::UNSPECIFIED))
        };

        let socket_addr = SocketAddr::new(ip_addr.unwrap_or(fallback_ip), port);

        // Return the address only if it's a loopback address
        if socket_addr.ip().is_loopback() {
            Some(socket_addr)
        } else {
            Some(SocketAddr::new(fallback_ip, port))
        }
    }

    /// Tries to open the authorization URL in the user's browser, and returns
    /// the obtained code.
    ///
    /// Note: this method requires the `cli` feature.
    pub fn get_code_from_user(&self, url: &str) -> ClientResult<String> {
        match webbrowser::open(url) {
            Ok(()) => println!("Opened {url} in your browser."),
            Err(why) => eprintln!(
                "Error when trying to open an URL in your browser: {why:?}. \
                 Please navigate here manually: {url}"
            ),
        }

        if let Some(addr) = self.get_socket_address() {
            self.get_authcode_listener(addr)
        } else {
            println!("Please enter the URL you were redirected to: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            self.parse_response_code(&input)
                .ok_or_else(|| ClientError::Cli("unable to parse the response code".to_owned()))
        }
    }

    /// Opens up the authorization URL in the user's browser so that it can
    /// authenticate. It reads from the standard input the redirect URI
    /// in order to obtain the access token information. The resulting access
    /// token will be saved internally once the operation is successful.
    ///
    /// If the [`Config::token_cached`] setting is enabled for this client,
    /// and a token exists in the cache, the token will be loaded and the client
    /// will attempt to automatically refresh the token if it is expired. If
    /// the token was unable to be refreshed, the client will then prompt the
    /// user for the token as normal.
    ///
    /// Note: this method requires the `cli` feature.
    ///
    /// [`Config::token_cached`]: crate::Config::token_cached
    pub fn prompt_for_token(&self, url: &str) -> ClientResult<()> {
        if let Ok(Some(new_token)) = self.read_token_cache(true) {
            let expired = new_token.is_expired();

            // Load token into client regardless of whether it's expired o
            // not, since it will be refreshed later anyway.
            *self.token.write() = Some(new_token);

            if expired {
                // Ensure that we actually got a token from the refetch
                if let Some(refreshed_token) = self.refetch_token()? {
                    *self.token.write() = Some(refreshed_token);
                } else {
                    error!("Unable to refresh expired token from token cache");
                    let code = self.get_code_from_user(url)?;
                    self.request_token(&code)?;
                }
            }
        } else {
            let code = self.get_code_from_user(url)?;
            self.request_token(&code)?;
        }

        self.write_token_cache()
    }

    /// Get current user playlists without required getting his profile.
    ///
    /// Parameters:
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    ///
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
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - track_ids - a list of track URIs, URLs or IDs
    /// - position - the position to add the items, a zero-based index
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/add-tracks-to-playlist)
    pub fn playlist_add_items(
        &self,
        playlist_id: &PlaylistId,
        items: &[&TrackId],
        position: Option<u32>,
    ) -> ClientResult<()> {
        let uris = items
            .iter()
            .map(|id| format!("spotify:track:{id}"))
            .collect::<Vec<_>>();
        let params = json!({
            "uris": uris,
            "position": position
        });

        let url = format!("playlists/{playlist_id}/tracks");
        let result = self.api_post(&url, &params)?;
        serde_json::from_str(&result).map_err(Into::into)
    }

    /// Removes all occurrences of the given items from the given playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - track_ids - the list of track ids to add to the playlist
    /// - snapshot_id - optional id of the playlist snapshot
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-playlist)
    pub fn playlist_remove_all_occurrences_of_items(
        &self,
        playlist_id: &PlaylistId,
        track_ids: &[&TrackId],
        snapshot_id: Option<&str>,
    ) -> ClientResult<()> {
        let tracks = track_ids
            .iter()
            .map(|id| {
                let mut map = Map::with_capacity(1);
                map.insert("uri".to_owned(), format!("spotify:track:{id}").into());
                map
            })
            .collect::<Vec<_>>();

        let params = json!({
            "tracks": tracks,
            "snapshot_id": snapshot_id
        });

        let url = format!("playlists/{playlist_id}/tracks");
        let result = self.api_delete(&url, &params)?;
        serde_json::from_str(&result).map_err(Into::into)
    }

    /// Remove one or more tracks from the current user's "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user)
    pub fn current_user_saved_tracks_delete(&self, track_ids: &[TrackId]) -> ClientResult<()> {
        let url = format!("me/tracks/?ids={}", track_ids.join(","));
        self.api_delete(&url, &Value::Null)?;

        Ok(())
    }

    /// Check if one or more tracks is already saved in the current Spotify
    /// user’s "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-tracks)
    pub fn current_user_saved_tracks_contains(
        &self,
        track_ids: &[TrackId],
    ) -> ClientResult<Vec<bool>> {
        let url = format!("me/tracks/contains/?ids={}", track_ids.join(","));
        let result = self.api_get(&url, &[])?;
        serde_json::from_str(&result).map_err(Into::into)
    }

    /// Save one or more tracks to the current user's "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user)
    pub fn current_user_saved_tracks_add(&self, track_ids: &[TrackId]) -> ClientResult<()> {
        let url = format!("me/tracks/?ids={}", track_ids.join(","));
        self.api_put(&url, &Value::Null)?;

        Ok(())
    }

    /// Get Information About The User’s Current Playback
    ///
    /// Parameters:
    /// - market: Optional. an ISO 3166-1 alpha-2 country code or the string from_token.
    /// - additional_types: Optional. A list of item types that your client
    ///   supports besides the default track type. Valid types are: `track` and
    ///   `episode`.
    ///
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
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-queue)
    pub fn current_user_queue(&self) -> ClientResult<CurrentUserQueue> {
        let result = self.api_get("me/player/queue", &[])?;
        serde_json::from_str(&result).map_err(Into::into)
    }

    /// Pause a User’s Playback.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/pause-a-users-playback)
    pub fn pause_playback(&self, device_id: Option<&str>) -> ClientResult<()> {
        let url = append_device_id("me/player/pause", device_id);
        self.api_put(&url, &Value::Null)?;

        Ok(())
    }

    /// Resume a User’s Playback.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    /// - position
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback)
    pub fn resume_playback(
        &self,
        device_id: Option<&str>,
        position: Option<chrono::Duration>,
    ) -> ClientResult<()> {
        let params = position.map_or(Value::Null, |position| {
            json!({
                "position_ms": position.num_milliseconds()
            })
        });

        let url = append_device_id("me/player/play", device_id);
        self.api_put(&url, &params)?;

        Ok(())
    }

    /// Skip User’s Playback To Next Track.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-next-track)
    pub fn next_track(&self, device_id: Option<&str>) -> ClientResult<()> {
        let url = append_device_id("me/player/next", device_id);
        self.api_post(&url, &Value::Null)?;

        Ok(())
    }

    /// Skip User’s Playback To Previous Track.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/skip-users-playback-to-previous-track)
    pub fn previous_track(&self, device_id: Option<&str>) -> ClientResult<()> {
        let url = append_device_id("me/player/previous", device_id);
        self.api_post(&url, &Value::Null)?;

        Ok(())
    }

    /// Seek To Position In Currently Playing Track.
    ///
    /// Parameters:
    /// - position - position in milliseconds to seek to
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/seek-to-position-in-currently-playing-track)
    pub fn seek_track(
        &self,
        position: chrono::Duration,
        device_id: Option<&str>,
    ) -> ClientResult<()> {
        let url = append_device_id(
            &format!("me/player/seek?position_ms={}", position.num_milliseconds()),
            device_id,
        );
        self.api_put(&url, &Value::Null)?;

        Ok(())
    }

    /// Set Volume For User’s Playback.
    ///
    /// Parameters:
    /// - volume_percent - volume between 0 and 100
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/set-volume-for-users-playback)
    pub fn volume(&self, volume_percent: u8, device_id: Option<&str>) -> ClientResult<()> {
        debug_assert!(
            volume_percent <= 100u8,
            "volume must be between 0 and 100, inclusive"
        );
        let url = append_device_id(
            &format!("me/player/volume?volume_percent={volume_percent}"),
            device_id,
        );
        self.api_put(&url, &Value::Null)?;

        Ok(())
    }

    /// Re-authenticate the client automatically if it's configured to do so,
    /// which uses the refresh token to obtain a new access token.
    pub fn auto_reauth(&self) -> ClientResult<()> {
        // NOTE: It's important to not leave the token locked, or else a
        // deadlock when calling `refresh_token` will occur.
        let should_reauth = self.token.read().as_ref().is_some_and(Token::is_expired);
        if should_reauth {
            self.refresh_token()
        } else {
            Ok(())
        }
    }

    /// Refreshes the current access token given a refresh token. The obtained
    /// token will be saved internally.
    pub fn refresh_token(&self) -> ClientResult<()> {
        let token = self.refetch_token()?;
        *self.token.write() = token;
        self.write_token_cache()
    }

    /// The headers required for authenticated requests to the API.
    ///
    /// Since this is accessed by authenticated requests always, it's where the
    /// automatic reauthentication takes place, if enabled.
    pub fn auth_headers(&self) -> ClientResult<(String, String)> {
        self.auto_reauth()?;

        Ok((
            "authorization".to_owned(),
            format!(
                "Bearer {}",
                self.token
                    .read()
                    .as_ref()
                    .ok_or(ClientError::InvalidToken)?
                    .access
            ),
        ))
    }

    // HTTP-related methods for the Spotify client. They wrap up the basic HTTP
    // client with its specific usage for endpoints or authentication.

    /// Convenience method to send GET requests related to an endpoint in the  API.
    pub fn api_get(&self, url: &str, payload: &[(&str, Option<&str>)]) -> ClientResult<String> {
        let mut request = self.http.get(api_url(url));
        let (key, val) = self.auth_headers()?;
        request = request.header(key, val);
        for (key, val) in payload {
            if let Some(val) = val {
                request = request.query(key, val);
            }
        }
        let response = request.call()?;
        Ok(response.into_body().read_to_string()?)
    }

    /// Convenience method to send POST requests related to an endpoint in the API.
    pub fn api_post(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let mut request = self.http.post(api_url(url));
        let (key, val) = self.auth_headers()?;
        request = request.header(key, val);
        let response = request.send_json(payload)?;
        Ok(response.into_body().read_to_string()?)
    }

    /// Convenience method to send PUT requests related to an endpoint in the API.
    pub fn api_put(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let mut request = self.http.put(&api_url(url));
        let (key, val) = self.auth_headers()?;
        request = request.header(key, val);
        let response = request.send_json(payload)?;
        Ok(response.into_body().read_to_string()?)
    }

    /// Convenience method to send DELETE requests related to an endpoint in the API.
    pub fn api_delete(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let mut request = self.http.delete(&api_url(url)).force_send_body();
        let (key, val) = self.auth_headers()?;
        request = request.header(key, val);
        let response = request.send_json(payload)?;
        Ok(response.into_body().read_to_string()?)
    }

    /// Updates the cache file at the internal cache path.
    ///
    /// This should be used whenever it's possible to, even if the cached token
    /// isn't configured, because this will already check `Config::token_cached`
    /// and do nothing in that case already.
    pub fn write_token_cache(&self) -> ClientResult<()> {
        if let Some(tok) = self.token.read().as_ref() {
            tok.write_cache(&self.config.cache_path)?;
        }

        Ok(())
    }

    /// Sends a request to Spotify for an access token.
    pub fn fetch_access_token(&self, payload: &[(&str, &str)]) -> ClientResult<Token> {
        let response = self
            .http
            .post("https://accounts.spotify.com/api/token".to_owned())
            .send_form(payload.to_owned())?
            .into_body()
            .read_to_string()?;
        let mut tok = serde_json::from_str::<Token>(&response)?;
        tok.expires_at = Utc::now().checked_add_signed(tok.expires_in);
        Ok(tok)
    }

    /// Returns a list of artists given the artist IDs, URIs, or URLs.
    ///
    /// Parameters:
    /// - artist_ids - a list of artist IDs, URIs or URLs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-artists)
    pub fn artists(&self, artist_ids: &[ArtistId]) -> ClientResult<Vec<Artist>> {
        let ids = artist_ids.join(",");
        let url = format!("artists/?ids={ids}");
        let result = self.api_get(&url, &[])?;

        serde_json::from_str::<Artists>(&result)
            .map_err(Into::into)
            .map(|x| x.artists)
    }

    /// Get full details of the items of a playlist owned by a user.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - fields - which fields to return
    /// - limit - the maximum number of tracks to return
    /// - offset - the index of the first track to return
    /// - market - an ISO 3166-1 alpha-2 country code or the string from_token.
    ///
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
        let params = [
            ("fields", fields),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ];

        let url = format!("playlists/{playlist_id}/tracks");
        let result = self.api_get(&url, &params)?;
        serde_json::from_str(&result).map_err(Into::into)
    }

    pub fn refetch_token(&self) -> ClientResult<Option<Token>> {
        match self.token.read().as_ref() {
            Some(Token {
                refresh: Some(refresh_token),
                ..
            }) => {
                let token = self.fetch_access_token(&[
                    ("grant_type", "refresh_token"),
                    ("refresh_token", refresh_token),
                    ("client_id", &self.creds.id),
                ])?;

                if let Some(callback_fn) = &*self.config.token_callback_fn.clone() {
                    callback_fn.0(token.clone())?;
                }

                Ok(Some(token))
            }
            _ => Ok(None),
        }
    }

    /// Note that the code verifier must be set at this point, either manually
    /// or with [`Self::get_authorize_url`]. Otherwise, this function will
    /// panic.
    pub fn request_token(&self, code: &str) -> ClientResult<()> {
        let verifier = self.verifier.as_ref().expect(
            "Unknown code verifier. Try calling \
            `AuthCodePkceSpotify::get_authorize_url` first or setting it \
            yourself.",
        );

        let token = self.fetch_access_token(&[
            ("client_id", self.creds.id.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.oauth.redirect_uri),
            ("code_verifier", verifier),
        ])?;

        if let Some(callback_fn) = &*self.config.token_callback_fn.clone() {
            callback_fn.0(token.clone())?;
        }

        *self.token.write() = Some(token);

        self.write_token_cache()
    }

    /// Same as [`Self::new`] but with an extra parameter to configure the client.
    pub fn with_config(creds: Credentials, oauth: OAuth, config: Config) -> Self {
        Self {
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }

    /// Generate the verifier code and the challenge code.
    pub fn generate_codes() -> (String, String) {
        // The code verifier is just the randomly generated string.
        let verifier = generate_random_string(VERIFIER_BYTES, alphabets::PKCE_CODE_VERIFIER);
        // The code challenge is the code verifier hashed with SHA256 and then
        // encoded with base64url.
        //
        // NOTE: base64url != base64; it uses a different set of characters. See
        // https://datatracker.ietf.org/doc/html/rfc4648#section-5 for more
        // information.
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge = hasher.finalize();

        let challenge = URL_SAFE_NO_PAD.encode(challenge);

        (verifier, challenge)
    }

    /// Returns the URL needed to authorize the current client as the first step
    /// in the authorization flow.
    ///
    /// [reference]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
    /// [rfce]: https://datatracker.ietf.org/doc/html/rfc7636#section-4.1
    pub fn get_authorize_url(&mut self) -> ClientResult<String> {
        let scopes = self
            .oauth
            .scopes
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" ");
        let (verifier, challenge) = Self::generate_codes();
        // The verifier will be needed later when requesting the token
        self.verifier = Some(verifier);

        let parsed = Url::parse_with_params(
            "https://accounts.spotify.com/authorize",
            &[
                ("client_id", self.creds.id.as_str()),
                ("response_type", "code"),
                ("redirect_uri", self.oauth.redirect_uri.as_str()),
                ("code_challenge_method", "S256"),
                ("code_challenge", challenge.as_str()),
                ("state", self.oauth.state.as_str()),
                ("scope", scopes.as_str()),
            ],
        )?;
        Ok(parsed.into())
    }
}
