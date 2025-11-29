use crate::rspotify::{
    ClientError, ClientResult, Config, Credentials, OAuth, Token, alphabets, auth_urls,
    clients::{append_device_id, convert_result},
    generate_random_string,
    http::{Form, Headers, HttpClient, Query},
    join_ids, join_scopes,
    model::{
        category::{Category, PageCategory},
        idtypes::{PlayContextId, PlayableId},
        offset::Offset,
        recommend::{Recommendations, RecommendationsAttribute},
        *,
    },
    params,
    util::{JsonBuilder, build_map},
};
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener},
    ops::Not,
    sync::{Arc, Mutex},
};
use url::Url;

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
#[derive(Clone, Debug, Default)]
pub struct SpotifyClient {
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Arc<Mutex<Option<Token>>>,
    /// The code verifier for the authentication process
    pub verifier: Option<String>,
    pub(crate) http: HttpClient,
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
        if !self.get_config().token_cached {
            tracing::info!("Auth token cache read ignored (not configured)");
            return Ok(None);
        }

        tracing::info!("Reading auth token cache");
        let token = Token::from_cache(&self.get_config().cache_path)?;
        if !self.get_oauth().scopes.is_subset(&token.scopes)
            || (!allow_expired && token.is_expired())
        {
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
        let expected_state = &self.get_oauth().state;
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
        tracing::info!("OAuth server listening on {:?}", socket_address);

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
        let redirect_full_url = format!("{}{}", self.get_oauth().redirect_uri, redirect_url);
        tracing::info!("redirect_full_url {}", redirect_full_url);

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
    pub fn get_socket_address(&self, redirect_url: &str) -> Option<SocketAddr> {
        let (host, port) = {
            let parsed_url = Url::parse(redirect_url).ok()?;
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
        tracing::info!("Opening brower with auth URL");
        match webbrowser::open(url) {
            Ok(_) => println!("Opened {url} in your browser."),
            Err(why) => eprintln!(
                "Error when trying to open an URL in your browser: {why:?}. \
                 Please navigate here manually: {url}"
            ),
        }

        if let Some(addr) = self.get_socket_address(&self.get_oauth().redirect_uri) {
            self.get_authcode_listener(addr)
        } else {
            tracing::info!("Prompting user for code");
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
        match self.read_token_cache(true) {
            Ok(Some(new_token)) => {
                let expired = new_token.is_expired();

                // Load token into client regardless of whether it's expired o
                // not, since it will be refreshed later anyway.
                *self.get_token().lock().unwrap() = Some(new_token);

                if expired {
                    // Ensure that we actually got a token from the refetch
                    match self.refetch_token()? {
                        Some(refreshed_token) => {
                            tracing::info!("Successfully refreshed expired token from token cache");
                            *self.get_token().lock().unwrap() = Some(refreshed_token)
                        }
                        // If not, prompt the user for it
                        None => {
                            tracing::info!("Unable to refresh expired token from token cache");
                            let code = self.get_code_from_user(url)?;
                            self.request_token(&code)?;
                        }
                    }
                }
            }
            // Otherwise following the usual procedure to get the token.
            _ => {
                let code = self.get_code_from_user(url)?;
                self.request_token(&code)?;
            }
        }

        self.write_token_cache()
    }

    /// The manually paginated version of [`Self::current_user_playlists`].
    pub fn current_user_playlists_manual(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedPlaylist>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map([("limit", limit.as_deref()), ("offset", offset.as_deref())]);

        let result = self.api_get("me/playlists", &params)?;
        convert_result(&result)
    }

    /// Creates a playlist for a user.
    ///
    /// Parameters:
    /// - user_id - the id of the user
    /// - name - the name of the playlist
    /// - public - is the created playlist public
    /// - description - the description of the playlist
    /// - collaborative - if the playlist will be collaborative. Note: to create a collaborative playlist you must also set public to false
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/create-playlist)
    pub fn user_playlist_create(
        &self,
        user_id: UserId<'_>,
        name: &str,
        public: Option<bool>,
        collaborative: Option<bool>,
        description: Option<&str>,
    ) -> ClientResult<FullPlaylist> {
        debug_assert!(
            !(collaborative.unwrap_or(false) && public.unwrap_or(false)),
            "To create a collaborative playlist you must also set public to \
            false. See the reference for more information."
        );

        let params = JsonBuilder::new()
            .required("name", name)
            .optional("public", public)
            .optional("collaborative", collaborative)
            .optional("description", description)
            .build();

        let url = format!("users/{}/playlists", user_id.id());
        let result = self.api_post(&url, &params)?;
        convert_result(&result)
    }

    /// Changes a playlist's name and/or public/private state.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - name - optional name of the playlist
    /// - public - optional is the playlist public
    /// - collaborative - optional is the playlist collaborative
    /// - description - optional description of the playlist
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/change-playlist-details)
    pub fn playlist_change_detail(
        &self,
        playlist_id: PlaylistId<'_>,
        name: Option<&str>,
        public: Option<bool>,
        description: Option<&str>,
        collaborative: Option<bool>,
    ) -> ClientResult<String> {
        let params = JsonBuilder::new()
            .optional("name", name)
            .optional("public", public)
            .optional("collaborative", collaborative)
            .optional("description", description)
            .build();

        let url = format!("playlists/{}", playlist_id.id());
        self.api_put(&url, &params)
    }

    /// Unfollows (deletes) a playlist for a user.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/unfollow-playlist)
    pub fn playlist_unfollow(&self, playlist_id: PlaylistId<'_>) -> ClientResult<()> {
        let url = format!("playlists/{}/followers", playlist_id.id());
        self.api_delete(&url, &json!({}))?;

        Ok(())
    }

    /// Adds items to a playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - track_ids - a list of track URIs, URLs or IDs
    /// - position - the position to add the items, a zero-based index
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/add-tracks-to-playlist)
    pub fn playlist_add_items<'a>(
        &self,
        playlist_id: PlaylistId<'_>,
        items: impl IntoIterator<Item = PlayableId<'a>> + Send + 'a,
        position: Option<u32>,
    ) -> ClientResult<PlaylistResult> {
        let uris = items.into_iter().map(|id| id.uri()).collect::<Vec<_>>();
        let params = JsonBuilder::new()
            .required("uris", uris)
            .optional("position", position)
            .build();

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.api_post(&url, &params)?;
        convert_result(&result)
    }

    /// Replace all items in a playlist
    ///
    /// Parameters:
    /// - user - the id of the user
    /// - playlist_id - the id of the playlist
    /// - tracks - the list of track ids to add to the playlist
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/reorder-or-replace-playlists-tracks)
    pub fn playlist_replace_items<'a>(
        &self,
        playlist_id: PlaylistId<'_>,
        items: impl IntoIterator<Item = PlayableId<'a>> + Send + 'a,
    ) -> ClientResult<()> {
        let uris = items.into_iter().map(|id| id.uri()).collect::<Vec<_>>();
        let params = JsonBuilder::new().required("uris", uris).build();

        let url = format!("playlists/{}/tracks", playlist_id.id());
        self.api_put(&url, &params)?;

        Ok(())
    }

    /// Reorder items in a playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - uris - a list of Spotify URIs to replace or clear
    /// - range_start - the position of the first track to be reordered
    /// - insert_before - the position where the tracks should be inserted
    /// - range_length - optional the number of tracks to be reordered (default:
    ///   1)
    /// - snapshot_id - optional playlist's snapshot ID
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/reorder-or-replace-playlists-tracks)
    pub fn playlist_reorder_items(
        &self,
        playlist_id: PlaylistId<'_>,
        range_start: Option<i32>,
        insert_before: Option<i32>,
        range_length: Option<u32>,
        snapshot_id: Option<&str>,
    ) -> ClientResult<PlaylistResult> {
        let params = JsonBuilder::new()
            .optional("range_start", range_start)
            .optional("insert_before", insert_before)
            .optional("range_length", range_length)
            .optional("snapshot_id", snapshot_id)
            .build();

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.api_put(&url, &params)?;
        convert_result(&result)
    }

    /// Removes all occurrences of the given items from the given playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - track_ids - the list of track ids to add to the playlist
    /// - snapshot_id - optional id of the playlist snapshot
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-playlist)
    pub fn playlist_remove_all_occurrences_of_items<'a>(
        &self,
        playlist_id: PlaylistId<'_>,
        track_ids: impl IntoIterator<Item = PlayableId<'a>> + Send + 'a,
        snapshot_id: Option<&str>,
    ) -> ClientResult<PlaylistResult> {
        let tracks = track_ids
            .into_iter()
            .map(|id| {
                let mut map = Map::with_capacity(1);
                map.insert("uri".to_owned(), id.uri().into());
                map
            })
            .collect::<Vec<_>>();

        let params = JsonBuilder::new()
            .required("tracks", tracks)
            .optional("snapshot_id", snapshot_id)
            .build();

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.api_delete(&url, &params)?;
        convert_result(&result)
    }

    /// Removes specfic occurrences of the given items from the given playlist.
    ///
    /// Parameters:
    /// - playlist_id: the id of the playlist
    /// - tracks: an array of map containing Spotify URIs of the tracks to
    ///   remove with their current positions in the playlist. For example:
    ///
    /// ```json
    /// {
    ///    "tracks":[
    ///       {
    ///          "uri":"spotify:track:4iV5W9uYEdYUVa79Axb7Rh",
    ///          "positions":[
    ///             0,
    ///             3
    ///          ]
    ///       },
    ///       {
    ///          "uri":"spotify:track:1301WleyT98MSxVHPZCA6M",
    ///          "positions":[
    ///             7
    ///          ]
    ///       }
    ///    ]
    /// }
    /// ```
    /// - snapshot_id: optional id of the playlist snapshot
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-playlist)
    pub fn playlist_remove_specific_occurrences_of_items<'a>(
        &self,
        playlist_id: PlaylistId<'_>,
        items: impl IntoIterator<Item = ItemPositions<'a>> + Send + 'a,
        snapshot_id: Option<&str>,
    ) -> ClientResult<PlaylistResult> {
        let tracks = items
            .into_iter()
            .map(|track| {
                let mut map = Map::new();
                map.insert("uri".to_owned(), track.id.uri().into());
                map.insert("positions".to_owned(), json!(track.positions));
                map
            })
            .collect::<Vec<_>>();

        let params = JsonBuilder::new()
            .required("tracks", tracks)
            .optional("snapshot_id", snapshot_id)
            .build();

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.api_delete(&url, &params)?;
        convert_result(&result)
    }

    /// Add the current authenticated user as a follower of a playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/follow-playlist)
    pub fn playlist_follow(
        &self,
        playlist_id: PlaylistId<'_>,
        public: Option<bool>,
    ) -> ClientResult<()> {
        let url = format!("playlists/{}/followers", playlist_id.id());

        let params = JsonBuilder::new().optional("public", public).build();

        self.api_put(&url, &params)?;

        Ok(())
    }

    /// Get detailed profile information about the current user.
    /// An alias for the 'current_user' method.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-current-users-profile)
    pub fn me(&self) -> ClientResult<PrivateUser> {
        let result = self.api_get("me/", &Query::new())?;
        convert_result(&result)
    }

    /// Get detailed profile information about the current user.
    /// An alias for the 'me' method.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-current-users-profile)
    pub fn current_user(&self) -> ClientResult<PrivateUser> {
        self.me()
    }

    /// Get information about the current users currently playing item.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-the-users-currently-playing-track)
    pub fn current_user_playing_item(&self) -> ClientResult<Option<CurrentlyPlayingContext>> {
        let result = self.api_get("me/player/currently-playing", &Query::new())?;
        if result.is_empty() {
            Ok(None)
        } else {
            convert_result(&result)
        }
    }

    /// The manually paginated version of [`Self::current_user_saved_tracks`].
    pub fn current_user_saved_tracks_manual(
        &self,
        market: Option<Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SavedTrack>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map([
            ("market", market.map(Into::into)),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ]);

        let result = self.api_get("me/tracks", &params)?;
        convert_result(&result)
    }

    /// Gets a list of the artists followed by the current authorized user.
    ///
    /// Parameters:
    /// - after - the last artist ID retrieved from the previous request
    /// - limit - the number of tracks to return
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-followed)
    pub fn current_user_followed_artists(
        &self,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> ClientResult<CursorBasedPage<FullArtist>> {
        let limit = limit.map(|s| s.to_string());
        let params = build_map([
            ("type", Some(Type::Artist.into())),
            ("after", after),
            ("limit", limit.as_deref()),
        ]);

        let result = self.api_get("me/following", &params)?;
        convert_result::<CursorPageFullArtists>(&result).map(|x| x.artists)
    }

    /// Remove one or more tracks from the current user's "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/remove-tracks-user)
    pub fn current_user_saved_tracks_delete<'a>(
        &self,
        track_ids: impl IntoIterator<Item = TrackId<'a>> + Send + 'a,
    ) -> ClientResult<()> {
        let url = format!("me/tracks/?ids={}", join_ids(track_ids));
        self.api_delete(&url, &json!({}))?;

        Ok(())
    }

    /// Check if one or more tracks is already saved in the current Spotify
    /// user’s "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/check-users-saved-tracks)
    pub fn current_user_saved_tracks_contains<'a>(
        &self,
        track_ids: impl IntoIterator<Item = TrackId<'a>> + Send + 'a,
    ) -> ClientResult<Vec<bool>> {
        let url = format!("me/tracks/contains/?ids={}", join_ids(track_ids));
        let result = self.api_get(&url, &Query::new())?;
        convert_result(&result)
    }

    /// Save one or more tracks to the current user's "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/save-tracks-user)
    pub fn current_user_saved_tracks_add<'a>(
        &self,
        track_ids: impl IntoIterator<Item = TrackId<'a>> + Send + 'a,
    ) -> ClientResult<()> {
        let url = format!("me/tracks/?ids={}", join_ids(track_ids));
        self.api_put(&url, &json!({}))?;

        Ok(())
    }

    /// Get a User’s Available Devices
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-users-available-devices)
    pub fn device(&self) -> ClientResult<Vec<Device>> {
        let result = self.api_get("me/player/devices", &Query::new())?;
        convert_result::<DevicePayload>(&result).map(|x| x.devices)
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
    pub fn current_playback<'a>(
        &self,
        country: Option<Market>,
        additional_types: Option<impl IntoIterator<Item = &'a AdditionalType> + Send + 'a>,
    ) -> ClientResult<Option<CurrentPlaybackContext>> {
        let additional_types = additional_types.map(|x| {
            x.into_iter()
                .map(Into::into)
                .collect::<Vec<&'static str>>()
                .join(",")
        });
        let params = build_map([
            ("country", country.map(Into::into)),
            ("additional_types", additional_types.as_deref()),
        ]);

        let result = self.api_get("me/player", &params)?;
        if result.is_empty() {
            Ok(None)
        } else {
            convert_result(&result)
        }
    }

    /// Get the User’s Currently Playing Track
    ///
    /// Parameters:
    /// - market: Optional. an ISO 3166-1 alpha-2 country code or the string from_token.
    /// - additional_types: Optional. A comma-separated list of item types that
    ///   your client supports besides the default track type. Valid types are:
    ///   `track` and `episode`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/get-the-users-currently-playing-track)
    pub fn current_playing<'a>(
        &'a self,
        market: Option<Market>,
        additional_types: Option<impl IntoIterator<Item = &'a AdditionalType> + Send + 'a>,
    ) -> ClientResult<Option<CurrentlyPlayingContext>> {
        let additional_types = additional_types.map(|x| {
            x.into_iter()
                .map(Into::into)
                .collect::<Vec<&'static str>>()
                .join(",")
        });
        let params = build_map([
            ("market", market.map(Into::into)),
            ("additional_types", additional_types.as_deref()),
        ]);

        let result = self.api_get("me/player/currently-playing", &params)?;
        if result.is_empty() {
            Ok(None)
        } else {
            convert_result(&result)
        }
    }

    /// Get the Current User’s Queue
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-queue)
    pub fn current_user_queue(&self) -> ClientResult<CurrentUserQueue> {
        let params = build_map([]);
        let result = self.api_get("me/player/queue", &params)?;
        convert_result(&result)
    }

    /// Transfer a User’s Playback.
    ///
    /// Note: Although an array is accepted, only a single device_id is
    /// currently supported. Supplying more than one will return 400 Bad Request
    ///
    /// Parameters:
    /// - device_id - transfer playback to this device
    /// - force_play - true: after transfer, play. false: keep current state.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/transfer-a-users-playback)
    pub fn transfer_playback(&self, device_id: &str, play: Option<bool>) -> ClientResult<()> {
        let params = JsonBuilder::new()
            .required("device_ids", [device_id])
            .optional("play", play)
            .build();

        self.api_put("me/player", &params)?;
        Ok(())
    }

    /// Start/Resume a User’s Playback.
    ///
    /// Provide a `context_uri` to start playback or a album, artist, or
    /// playlist. Provide a `uris` list to start playback of one or more tracks.
    /// Provide `offset` as `{"position": <int>}` or `{"uri": "<track uri>"}` to
    /// start playback at a particular offset.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    /// - context_uri - spotify context uri to play
    /// - uris - spotify track uris
    /// - offset - offset into context by index or track
    /// - position - Indicates from what position to start playback.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback)
    pub fn start_context_playback(
        &self,
        context_uri: PlayContextId<'_>,
        device_id: Option<&str>,
        offset: Option<Offset>,
        position: Option<chrono::Duration>,
    ) -> ClientResult<()> {
        let params = JsonBuilder::new()
            .required("context_uri", context_uri.uri())
            .optional(
                "offset",
                offset.map(|x| match x {
                    Offset::Position(position) => {
                        json!({ "position": position.num_milliseconds() })
                    }
                    Offset::Uri(uri) => json!({ "uri": uri }),
                }),
            )
            .optional("position_ms", position.map(|p| p.num_milliseconds()))
            .build();

        let url = append_device_id("me/player/play", device_id);
        self.api_put(&url, &params)?;

        Ok(())
    }

    /// Start a user's playback
    ///
    /// Parameters:
    /// - uris
    /// - device_id
    /// - offset
    /// - position
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/start-a-users-playback)
    pub fn start_uris_playback<'a>(
        &self,
        uris: impl IntoIterator<Item = PlayableId<'a>> + Send + 'a,
        device_id: Option<&str>,
        offset: Option<Offset>,
        position: Option<chrono::Duration>,
    ) -> ClientResult<()> {
        let params = JsonBuilder::new()
            .required(
                "uris",
                uris.into_iter().map(|id| id.uri()).collect::<Vec<_>>(),
            )
            .optional("position_ms", position.map(|p| p.num_milliseconds()))
            .optional(
                "offset",
                offset.map(|x| match x {
                    Offset::Position(position) => {
                        json!({ "position": position.num_milliseconds() })
                    }
                    Offset::Uri(uri) => json!({ "uri": uri }),
                }),
            )
            .build();

        let url = append_device_id("me/player/play", device_id);
        self.api_put(&url, &params)?;

        Ok(())
    }

    /// Pause a User’s Playback.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/pause-a-users-playback)
    pub fn pause_playback(&self, device_id: Option<&str>) -> ClientResult<()> {
        let url = append_device_id("me/player/pause", device_id);
        self.api_put(&url, &json!({}))?;

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
        let params = JsonBuilder::new()
            .optional("position_ms", position.map(|p| p.num_milliseconds()))
            .build();

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
        self.api_post(&url, &json!({}))?;

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
        self.api_post(&url, &json!({}))?;

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
        self.api_put(&url, &json!({}))?;

        Ok(())
    }

    /// Set Repeat Mode On User’s Playback.
    ///
    /// Parameters:
    /// - state - `track`, `context`, or `off`
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/set-repeat-mode-on-users-playback)
    pub fn repeat(&self, state: RepeatState, device_id: Option<&str>) -> ClientResult<()> {
        let url = append_device_id(
            &format!("me/player/repeat?state={}", <&str>::from(state)),
            device_id,
        );
        self.api_put(&url, &json!({}))?;

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
        self.api_put(&url, &json!({}))?;

        Ok(())
    }

    /// Toggle Shuffle For User’s Playback.
    ///
    /// Parameters:
    /// - state - true or false
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/toggle-shuffle-for-users-playback)
    pub fn shuffle(&self, state: bool, device_id: Option<&str>) -> ClientResult<()> {
        let url = append_device_id(&format!("me/player/shuffle?state={state}"), device_id);
        self.api_put(&url, &json!({}))?;

        Ok(())
    }

    /// Add an item to the end of the user's playback queue.
    ///
    /// Parameters:
    /// - uri - The uri of the item to add, Track or Episode
    /// - device id - The id of the device targeting
    /// - If no device ID provided the user's currently active device is
    ///   targeted
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/add-to-queue)
    pub fn add_item_to_queue(
        &self,
        item: PlayableId<'_>,
        device_id: Option<&str>,
    ) -> ClientResult<()> {
        let url = append_device_id(&format!("me/player/queue?uri={}", item.uri()), device_id);
        self.api_post(&url, &json!({}))?;

        Ok(())
    }

    /// Add a show or a list of shows to a user’s library.
    ///
    /// Parameters:
    /// - ids(Required) A comma-separated list of Spotify IDs for the shows to
    ///   be added to the user’s library.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/save-shows-user)
    pub fn save_shows<'a>(
        &self,
        show_ids: impl IntoIterator<Item = ShowId<'a>> + Send + 'a,
    ) -> ClientResult<()> {
        let url = format!("me/shows/?ids={}", join_ids(show_ids));
        self.api_put(&url, &json!({}))?;

        Ok(())
    }

    /// Returns the absolute URL for an endpoint in the API.
    pub fn api_url(&self, url: &str) -> String {
        let mut base = self.get_config().api_base_url.clone();
        if !base.ends_with('/') {
            base.push('/');
        }
        base + url
    }

    /// Returns the absolute URL for an authentication step in the API.
    pub fn auth_url(&self, url: &str) -> String {
        let mut base = self.get_config().auth_base_url.clone();
        if !base.ends_with('/') {
            base.push('/');
        }
        base + url
    }

    /// Re-authenticate the client automatically if it's configured to do so,
    /// which uses the refresh token to obtain a new access token.
    pub fn auto_reauth(&self) -> ClientResult<()> {
        if !self.get_config().token_refreshing {
            return Ok(());
        }

        // NOTE: It's important to not leave the token locked, or else a
        // deadlock when calling `refresh_token` will occur.
        let should_reauth = self
            .get_token()
            .lock()
            .unwrap()
            .as_ref()
            .is_some_and(Token::is_expired);

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
        *self.get_token().lock().unwrap() = token;
        self.write_token_cache()
    }

    /// The headers required for authenticated requests to the API.
    ///
    /// Since this is accessed by authenticated requests always, it's where the
    /// automatic reauthentication takes place, if enabled.
    #[doc(hidden)]
    pub fn auth_headers(&self) -> ClientResult<Headers> {
        self.auto_reauth()?;

        Ok(self
            .get_token()
            .lock()
            .unwrap()
            .as_ref()
            .ok_or(ClientError::InvalidToken)?
            .auth_headers())
    }

    // HTTP-related methods for the Spotify client. They wrap up the basic HTTP
    // client with its specific usage for endpoints or authentication.

    /// Convenience method to send GET requests related to an endpoint in the
    /// API.
    #[doc(hidden)]
    pub fn api_get(&self, url: &str, payload: &Query<'_>) -> ClientResult<String> {
        let url = self.api_url(url);
        let headers = self.auth_headers()?;
        Ok(self.get_http().get(&url, Some(&headers), payload)?)
    }

    /// Convenience method to send POST requests related to an endpoint in the
    /// API.
    #[doc(hidden)]
    pub fn api_post(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let url = self.api_url(url);
        let headers = self.auth_headers()?;
        Ok(self.get_http().post(&url, Some(&headers), payload)?)
    }

    /// Convenience method to send PUT requests related to an endpoint in the
    /// API.
    #[doc(hidden)]
    pub fn api_put(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let url = self.api_url(url);
        let headers = self.auth_headers()?;
        Ok(self.get_http().put(&url, Some(&headers), payload)?)
    }

    /// Convenience method to send DELETE requests related to an endpoint in the
    /// API.
    #[doc(hidden)]
    pub fn api_delete(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let url = self.api_url(url);
        let headers = self.auth_headers()?;
        Ok(self.get_http().delete(&url, Some(&headers), payload)?)
    }

    /// Convenience method to send POST requests related to the authentication
    /// process.
    #[doc(hidden)]
    #[inline]
    pub fn auth_post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'_>,
    ) -> ClientResult<String> {
        let url = self.auth_url(url);
        Ok(self.get_http().post_form(&url, headers, payload)?)
    }

    /// Updates the cache file at the internal cache path.
    ///
    /// This should be used whenever it's possible to, even if the cached token
    /// isn't configured, because this will already check `Config::token_cached`
    /// and do nothing in that case already.
    pub fn write_token_cache(&self) -> ClientResult<()> {
        if !self.get_config().token_cached {
            tracing::info!("Token cache write ignored (not configured)");
            return Ok(());
        }

        tracing::info!("Writing token cache");
        if let Some(tok) = self.get_token().lock().unwrap().as_ref() {
            tok.write_cache(&self.get_config().cache_path)?;
        }

        Ok(())
    }

    /// Sends a request to Spotify for an access token.
    pub fn fetch_access_token(
        &self,
        payload: &Form<'_>,
        headers: Option<&Headers>,
    ) -> ClientResult<Token> {
        let response = self.auth_post(auth_urls::TOKEN, headers, payload)?;

        let mut tok = serde_json::from_str::<Token>(&response)?;
        tok.expires_at = Utc::now().checked_add_signed(tok.expires_in);
        Ok(tok)
    }

    /// Returns a single track given the track's ID, URI or URL.
    ///
    /// Parameters:
    /// - track_id - a spotify URI, URL or ID
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-track)
    pub fn track(&self, track_id: TrackId<'_>, market: Option<Market>) -> ClientResult<FullTrack> {
        let params = build_map([("market", market.map(Into::into))]);

        let url = format!("tracks/{}", track_id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Returns a list of tracks given a list of track IDs, URIs, or URLs.
    ///
    /// Parameters:
    /// - track_ids - a list of spotify URIs, URLs or IDs
    /// - market - an ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-several-tracks)
    pub fn tracks<'a>(
        &self,
        track_ids: impl IntoIterator<Item = TrackId<'a>> + Send + 'a,
        market: Option<Market>,
    ) -> ClientResult<Vec<FullTrack>> {
        let ids = join_ids(track_ids);
        let params = build_map([("market", market.map(Into::into))]);

        let url = format!("tracks/?ids={ids}");
        let result = self.api_get(&url, &params)?;
        convert_result::<FullTracks>(&result).map(|x| x.tracks)
    }

    /// Returns a single artist given the artist's ID, URI or URL.
    ///
    /// Parameters:
    /// - artist_id - an artist ID, URI or URL
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artist)
    pub fn artist(&self, artist_id: ArtistId<'_>) -> ClientResult<FullArtist> {
        let url = format!("artists/{}", artist_id.id());
        let result = self.api_get(&url, &Query::new())?;
        convert_result(&result)
    }

    /// Returns a list of artists given the artist IDs, URIs, or URLs.
    ///
    /// Parameters:
    /// - artist_ids - a list of artist IDs, URIs or URLs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-artists)
    pub fn artists<'a>(
        &self,
        artist_ids: impl IntoIterator<Item = ArtistId<'a>> + Send + 'a,
    ) -> ClientResult<Vec<FullArtist>> {
        let ids = join_ids(artist_ids);
        let url = format!("artists/?ids={ids}");
        let result = self.api_get(&url, &Query::new())?;

        convert_result::<FullArtists>(&result).map(|x| x.artists)
    }

    /// Get Spotify catalog information about an artist's albums.
    ///
    /// Parameters:
    /// - artist_id - the artist ID, URI or URL
    /// - include_groups -  a list of album type like 'album', 'single' that will be used to filter response. if not supplied, all album types will be returned.
    /// - market - limit the response to one particular country.
    /// - limit  - the number of albums to return
    /// - offset - the index of the first album to return
    ///
    /// See [`Self::artist_albums_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-albums)
    pub fn artist_albums_manual<'a>(
        &self,
        artist_id: ArtistId<'_>,
        include_groups: impl IntoIterator<Item = AlbumType> + Send + 'a,
        market: Option<Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedAlbum>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let include_groups_vec = include_groups
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<&'static str>>();
        let include_groups_opt = include_groups_vec
            .is_empty()
            .not()
            .then_some(include_groups_vec)
            .map(|t| t.join(","));

        let params = build_map([
            ("include_groups", include_groups_opt.as_deref()),
            ("market", market.map(Into::into)),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ]);

        let url = format!("artists/{}/albums", artist_id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Get Spotify catalog information about an artist's top 10 tracks by
    /// country.
    ///
    /// Parameters:
    /// - artist_id - the artist ID, URI or URL
    /// - market - limit the response to one particular country.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-artists-top-tracks)
    pub fn artist_top_tracks(
        &self,
        artist_id: ArtistId<'_>,
        market: Option<Market>,
    ) -> ClientResult<Vec<FullTrack>> {
        let params = build_map([("market", market.map(Into::into))]);

        let url = format!("artists/{}/top-tracks", artist_id.id());
        let result = self.api_get(&url, &params)?;
        convert_result::<FullTracks>(&result).map(|x| x.tracks)
    }

    /// Returns a single album given the album's ID, URIs or URL.
    ///
    /// Parameters:
    /// - album_id - the album ID, URI or URL
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-album)
    pub fn album(&self, album_id: AlbumId<'_>, market: Option<Market>) -> ClientResult<FullAlbum> {
        let params = build_map([("market", market.map(Into::into))]);

        let url = format!("albums/{}", album_id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Returns a list of albums given the album IDs, URIs, or URLs.
    ///
    /// Parameters:
    /// - albums_ids - a list of album IDs, URIs or URLs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-albums)
    pub fn albums<'a>(
        &self,
        album_ids: impl IntoIterator<Item = AlbumId<'a>> + Send + 'a,
        market: Option<Market>,
    ) -> ClientResult<Vec<FullAlbum>> {
        let params = build_map([("market", market.map(Into::into))]);

        let ids = join_ids(album_ids);
        let url = format!("albums/?ids={ids}");
        let result = self.api_get(&url, &params)?;
        convert_result::<FullAlbums>(&result).map(|x| x.albums)
    }

    /// Get Spotify catalog information about an album's tracks.
    ///
    /// Parameters:
    /// - album_id - the album ID, URI or URL
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    ///
    /// See [`Self::album_track_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-albums-tracks)
    pub fn album_track_manual(
        &self,
        album_id: AlbumId<'_>,
        market: Option<Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedTrack>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map([
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
            ("market", market.map(Into::into)),
        ]);

        let url = format!("albums/{}/tracks", album_id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Gets basic profile information about a Spotify User.
    ///
    /// Parameters:
    /// - user - the id of the usr
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-users-profile)
    pub fn user(&self, user_id: UserId<'_>) -> ClientResult<PublicUser> {
        let url = format!("users/{}", user_id.id());
        let result = self.api_get(&url, &Query::new())?;
        convert_result(&result)
    }

    /// Get full details about Spotify playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - market - an ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlist)
    pub fn playlist(
        &self,
        playlist_id: PlaylistId<'_>,
        fields: Option<&str>,
        market: Option<Market>,
    ) -> ClientResult<FullPlaylist> {
        let params = build_map([("fields", fields), ("market", market.map(Into::into))]);

        let url = format!("playlists/{}", playlist_id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Gets playlist of a user.
    ///
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    /// - fields - which fields to return
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-list-users-playlists)
    pub fn user_playlist(
        &self,
        user_id: UserId<'_>,
        playlist_id: Option<PlaylistId<'_>>,
        fields: Option<&str>,
    ) -> ClientResult<FullPlaylist> {
        let params = build_map([("fields", fields)]);

        let url = match playlist_id {
            Some(playlist_id) => format!("users/{}/playlists/{}", user_id.id(), playlist_id.id()),
            None => format!("users/{}/starred", user_id.id()),
        };
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Check to see if the given users are following the given playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - user_ids - the ids of the users that you want to check to see if they
    ///   follow the playlist. Maximum: 5 ids.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/check-if-user-follows-playlist)
    pub fn playlist_check_follow(
        &self,
        playlist_id: PlaylistId<'_>,
        user_ids: &[UserId<'_>],
    ) -> ClientResult<Vec<bool>> {
        debug_assert!(
            user_ids.len() <= 5,
            "The maximum length of user ids is limited to 5 :-)"
        );
        let url = format!(
            "playlists/{}/followers/contains?ids={}",
            playlist_id.id(),
            user_ids.iter().map(Id::id).collect::<Vec<_>>().join(","),
        );
        let result = self.api_get(&url, &Query::new())?;
        convert_result(&result)
    }

    /// Get Spotify catalog information for a single show identified by its unique Spotify ID.
    ///
    /// Path Parameters:
    /// - id: The Spotify ID for the show.
    ///
    /// Query Parameters
    /// - market(Optional): An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-show)
    pub fn get_a_show(&self, id: ShowId<'_>, market: Option<Market>) -> ClientResult<FullShow> {
        let params = build_map([("market", market.map(Into::into))]);

        let url = format!("shows/{}", id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Get Spotify catalog information for multiple shows based on their
    /// Spotify IDs.
    ///
    /// Query Parameters
    /// - ids(Required) A comma-separated list of the Spotify IDs for the shows. Maximum: 50 IDs.
    /// - market(Optional) An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-shows)
    pub fn get_several_shows<'a>(
        &self,
        ids: impl IntoIterator<Item = ShowId<'a>> + Send + 'a,
        market: Option<Market>,
    ) -> ClientResult<Vec<SimplifiedShow>> {
        let ids = join_ids(ids);
        let params = build_map([("ids", Some(&ids)), ("market", market.map(Into::into))]);

        let result = self.api_get("shows", &params)?;
        convert_result::<SeversalSimplifiedShows>(&result).map(|x| x.shows)
    }

    /// Get Spotify catalog information about an show’s episodes. Optional
    /// parameters can be used to limit the number of episodes returned.
    ///
    /// Path Parameters
    /// - id: The Spotify ID for the show.
    ///
    /// Query Parameters
    /// - limit: Optional. The maximum number of episodes to return. Default: 20. Minimum: 1. Maximum: 50.
    /// - offset: Optional. The index of the first episode to return. Default: 0 (the first object). Use with limit to get the next set of episodes.
    /// - market: Optional. An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// See [`Self::get_shows_episodes_manual`] for a manually paginated version
    /// of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-shows-episodes)
    pub fn get_shows_episodes_manual(
        &self,
        id: ShowId<'_>,
        market: Option<Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedEpisode>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map([
            ("market", market.map(Into::into)),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ]);

        let url = format!("shows/{}/episodes", id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Get Spotify catalog information for a single episode identified by its unique Spotify ID.
    ///
    /// Path Parameters
    /// - id: The Spotify ID for the episode.
    ///
    /// Query Parameters
    /// - market: Optional. An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-an-episode)
    pub fn get_an_episode(
        &self,
        id: EpisodeId<'_>,
        market: Option<Market>,
    ) -> ClientResult<FullEpisode> {
        let url = format!("episodes/{}", id.id());
        let params = build_map([("market", market.map(Into::into))]);

        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Get Spotify catalog information for multiple episodes based on their Spotify IDs.
    ///
    /// Query Parameters
    /// - ids: Required. A comma-separated list of the Spotify IDs for the episodes. Maximum: 50 IDs.
    /// - market: Optional. An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-multiple-episodes)
    pub fn get_several_episodes<'a>(
        &self,
        ids: impl IntoIterator<Item = EpisodeId<'a>> + Send + 'a,
        market: Option<Market>,
    ) -> ClientResult<Vec<FullEpisode>> {
        let ids = join_ids(ids);
        let params = build_map([("ids", Some(&ids)), ("market", market.map(Into::into))]);

        let result = self.api_get("episodes", &params)?;
        convert_result::<EpisodesPayload>(&result).map(|x| x.episodes)
    }

    /// Get a list of new album releases featured in Spotify
    ///
    /// Parameters:
    /// - country - An ISO 3166-1 alpha-2 country code or string from_token.
    /// - locale - The desired language, consisting of an ISO 639 language code
    ///   and an ISO 3166-1 alpha-2 country code, joined by an underscore.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0 (the first
    ///   object). Use with limit to get the next set of items.
    ///
    /// See [`Self::categories_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-categories)
    pub fn categories_manual(
        &self,
        locale: Option<&str>,
        country: Option<Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<Category>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map([
            ("locale", locale),
            ("country", country.map(Into::into)),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ]);
        let result = self.api_get("browse/categories", &params)?;
        convert_result::<PageCategory>(&result).map(|x| x.categories)
    }

    /// Get a list of playlists in a category in Spotify
    ///
    /// Parameters:
    /// - category_id - The category id to get playlists from.
    /// - country - An ISO 3166-1 alpha-2 country code or the string from_token.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0 (the first
    ///   object). Use with limit to get the next set of items.
    ///
    /// See [`Self::category_playlists_manual`] for a manually paginated version
    /// of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-a-categories-playlists)
    pub fn category_playlists_manual(
        &self,
        category_id: &str,
        country: Option<Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedPlaylist>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map([
            ("country", country.map(Into::into)),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ]);

        let url = format!("browse/categories/{category_id}/playlists");
        let result = self.api_get(&url, &params)?;
        convert_result::<CategoryPlaylists>(&result).map(|x| x.playlists)
    }

    /// Get a list of Spotify featured playlists.
    ///
    /// Parameters:
    /// - locale - The desired language, consisting of a lowercase ISO 639
    ///   language code and an uppercase ISO 3166-1 alpha-2 country code,
    ///   joined by an underscore.
    /// - country - An ISO 3166-1 alpha-2 country code or the string from_token.
    /// - timestamp - A timestamp in ISO 8601 format: yyyy-MM-ddTHH:mm:ss. Use
    ///   this parameter to specify the user's local time to get results
    ///   tailored for that specific date and time in the day
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0
    ///   (the first object). Use with limit to get the next set of
    ///   items.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-featured-playlists)
    pub fn featured_playlists(
        &self,
        locale: Option<&str>,
        country: Option<Market>,
        timestamp: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<FeaturedPlaylists> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let timestamp = timestamp.map(|x| x.to_rfc3339());
        let params = build_map([
            ("locale", locale),
            ("country", country.map(Into::into)),
            ("timestamp", timestamp.as_deref()),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ]);

        let result = self.api_get("browse/featured-playlists", &params)?;
        convert_result(&result)
    }

    /// Get a list of new album releases featured in Spotify.
    ///
    /// Parameters:
    /// - country - An ISO 3166-1 alpha-2 country code or string from_token.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0 (the first
    ///   object). Use with limit to get the next set of items.
    ///
    /// See [`Self::new_releases_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-new-releases)
    pub fn new_releases_manual(
        &self,
        country: Option<Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedAlbum>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map([
            ("country", country.map(Into::into)),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ]);

        let result = self.api_get("browse/new-releases", &params)?;
        convert_result::<PageSimplifiedAlbums>(&result).map(|x| x.albums)
    }

    /// Get Recommendations Based on Seeds
    ///
    /// Parameters:
    /// - attributes - restrictions on attributes for the selected tracks, such
    ///   as `min_acousticness` or `target_duration_ms`.
    /// - seed_artists - a list of artist IDs, URIs or URLs
    /// - seed_tracks - a list of artist IDs, URIs or URLs
    /// - seed_genres - a list of genre names. Available genres for
    /// - market - An ISO 3166-1 alpha-2 country code or the string from_token.
    ///   If provided, all results will be playable in this country.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 100
    /// - `min/max/target_<attribute>` - For the tuneable track attributes
    ///   listed in the documentation, these values provide filters and
    ///   targeting on results.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-recommendations)
    pub fn recommendations<'a>(
        &self,
        attributes: impl IntoIterator<Item = RecommendationsAttribute> + Send + 'a,
        seed_artists: Option<impl IntoIterator<Item = ArtistId<'a>> + Send + 'a>,
        seed_genres: Option<impl IntoIterator<Item = &'a str> + Send + 'a>,
        seed_tracks: Option<impl IntoIterator<Item = TrackId<'a>> + Send + 'a>,
        market: Option<Market>,
        limit: Option<u32>,
    ) -> ClientResult<Recommendations> {
        let seed_artists = seed_artists.map(join_ids);
        let seed_genres = seed_genres.map(|x| x.into_iter().collect::<Vec<_>>().join(","));
        let seed_tracks = seed_tracks.map(join_ids);
        let limit = limit.map(|x| x.to_string());
        let mut params = build_map([
            ("seed_artists", seed_artists.as_deref()),
            ("seed_genres", seed_genres.as_deref()),
            ("seed_tracks", seed_tracks.as_deref()),
            ("market", market.map(Into::into)),
            ("limit", limit.as_deref()),
        ]);

        // First converting the attributes into owned `String`s
        let owned_attributes = attributes
            .into_iter()
            .map(|attr| (<&str>::from(attr).to_owned(), attr.value_string()))
            .collect::<HashMap<_, _>>();
        // Afterwards converting the values into `&str`s; otherwise they
        // wouldn't live long enough
        let borrowed_attributes = owned_attributes
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()));
        // And finally adding all of them to the payload
        params.extend(borrowed_attributes);

        let result = self.api_get("recommendations", &params)?;
        convert_result(&result)
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
    /// See [`Self::playlist_items_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-playlists-tracks)
    pub fn playlist_items_manual(
        &self,
        playlist_id: PlaylistId<'_>,
        fields: Option<&str>,
        market: Option<Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<PlaylistItem>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map([
            ("fields", fields),
            ("market", market.map(Into::into)),
            ("limit", limit.as_deref()),
            ("offset", offset.as_deref()),
        ]);

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    /// Gets playlists of a user.
    ///
    /// Parameters:
    /// - user_id - the id of the usr
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    ///
    /// See [`Self::user_playlists_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#/operations/get-list-users-playlists)
    pub fn user_playlists_manual(
        &self,
        user_id: UserId<'_>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedPlaylist>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map([("limit", limit.as_deref()), ("offset", offset.as_deref())]);

        let url = format!("users/{}/playlists", user_id.id());
        let result = self.api_get(&url, &params)?;
        convert_result(&result)
    }

    pub fn get_http(&self) -> &HttpClient {
        &self.http
    }

    pub fn get_token(&self) -> Arc<Mutex<Option<Token>>> {
        Arc::clone(&self.token)
    }

    pub fn get_creds(&self) -> &Credentials {
        &self.creds
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn refetch_token(&self) -> ClientResult<Option<Token>> {
        match self.token.lock().unwrap().as_ref() {
            Some(Token {
                refresh_token: Some(refresh_token),
                ..
            }) => {
                let mut data = Form::new();
                data.insert(params::GRANT_TYPE, params::GRANT_TYPE_REFRESH_TOKEN);
                data.insert(params::REFRESH_TOKEN, refresh_token);
                data.insert(params::CLIENT_ID, &self.creds.id);

                let token = self.fetch_access_token(&data, None)?;

                if let Some(callback_fn) = &*self.get_config().token_callback_fn.clone() {
                    callback_fn.0(token.clone())?;
                }

                Ok(Some(token))
            }
            _ => Ok(None),
        }
    }

    pub fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }

    /// Note that the code verifier must be set at this point, either manually
    /// or with [`Self::get_authorize_url`]. Otherwise, this function will
    /// panic.
    pub fn request_token(&self, code: &str) -> ClientResult<()> {
        tracing::info!("Requesting PKCE Auth Code token");

        let verifier = self.verifier.as_ref().expect(
            "Unknown code verifier. Try calling \
            `AuthCodePkceSpotify::get_authorize_url` first or setting it \
            yourself.",
        );

        let mut data = Form::new();
        data.insert(params::CLIENT_ID, &self.creds.id);
        data.insert(params::GRANT_TYPE, params::GRANT_TYPE_AUTH_CODE);
        data.insert(params::CODE, code);
        data.insert(params::REDIRECT_URI, &self.oauth.redirect_uri);
        data.insert(params::CODE_VERIFIER, verifier);

        let token = self.fetch_access_token(&data, None)?;

        if let Some(callback_fn) = &*self.get_config().token_callback_fn.clone() {
            callback_fn.0(token.clone())?;
        }

        *self.token.lock().unwrap() = Some(token);

        self.write_token_cache()
    }

    /// Builds a new [`AuthCodePkceSpotify`] given a pair of client credentials and OAuth information.
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        Self {
            creds,
            oauth,
            ..Default::default()
        }
    }

    /// Build a new [`AuthCodePkceSpotify`] from an already generated token.
    /// Note that once the token expires this will fail to make requests, as the client credentials aren't known.
    pub fn from_token(token: Token) -> Self {
        Self {
            token: Arc::new(Mutex::new(Some(token))),
            ..Default::default()
        }
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

    /// Build a new [`AuthCodePkceSpotify`] from an already generated token and config. Use this to be able to refresh a token.
    pub fn from_token_with_config(
        token: Token,
        creds: Credentials,
        oauth: OAuth,
        config: Config,
    ) -> Self {
        Self {
            token: Arc::new(Mutex::new(Some(token))),
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }

    /// Generate the verifier code and the challenge code.
    pub fn generate_codes(verifier_bytes: usize) -> (String, String) {
        tracing::info!("Generating PKCE codes");

        debug_assert!(verifier_bytes >= 43);
        debug_assert!(verifier_bytes <= 128);
        // The code verifier is just the randomly generated string.
        let verifier = generate_random_string(verifier_bytes, alphabets::PKCE_CODE_VERIFIER);
        // The code challenge is the code verifier hashed with SHA256 and then
        // encoded with base64url.
        //
        // NOTE: base64url != base64; it uses a different set of characters. See
        // https://datatracker.ietf.org/doc/html/rfc4648#section-5 for more
        // information.
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge = hasher.finalize();

        let challenge = general_purpose::URL_SAFE_NO_PAD.encode(challenge);

        (verifier, challenge)
    }

    /// Returns the URL needed to authorize the current client as the first step
    /// in the authorization flow.
    ///
    /// The parameter `verifier_bytes` is the length of the randomly generated
    /// code verifier. Note that it must be between 43 and 128. If `None` is
    /// given, a length of 43 will be used by default. See [the official
    /// docs][reference] or [PKCE's RFC][rfce] for more information about the
    /// code verifier.
    ///
    /// [reference]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
    /// [rfce]: https://datatracker.ietf.org/doc/html/rfc7636#section-4.1
    pub fn get_authorize_url(&mut self, verifier_bytes: Option<usize>) -> ClientResult<String> {
        tracing::info!("Building auth URL");

        let scopes = join_scopes(&self.oauth.scopes);
        let verifier_bytes = verifier_bytes.unwrap_or(43);
        let (verifier, challenge) = Self::generate_codes(verifier_bytes);
        // The verifier will be needed later when requesting the token
        self.verifier = Some(verifier);

        let mut payload: HashMap<&str, &str> = HashMap::new();
        payload.insert(params::CLIENT_ID, &self.creds.id);
        payload.insert(params::RESPONSE_TYPE, params::RESPONSE_TYPE_CODE);
        payload.insert(params::REDIRECT_URI, &self.oauth.redirect_uri);
        payload.insert(
            params::CODE_CHALLENGE_METHOD,
            params::CODE_CHALLENGE_METHOD_S256,
        );
        payload.insert(params::CODE_CHALLENGE, &challenge);
        payload.insert(params::STATE, &self.oauth.state);
        payload.insert(params::SCOPE, &scopes);

        let request_url = self.auth_url(auth_urls::AUTHORIZE);
        let parsed = Url::parse_with_params(&request_url, payload)?;
        Ok(parsed.into())
    }
}
