use super::{
    CLIENT_ID, ClientResult, SPOTIFY_SESSION_CACHE, client_error, config_path, random_token, session,
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use parking_lot::Mutex;
use ring::digest::{SHA256, digest};
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    sync::Arc,
};
use tracing::{info, warn};
use ureq::{
    Agent,
    http::{Method, Request},
};

const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const REDIRECT_ADDR: &str = "127.0.0.1:8898";
const REDIRECT_URI: &str = "http://127.0.0.1:8898/login";
const SCOPES: &str = "streaming app-remote-control";

#[derive(Deserialize)]
struct OAuthToken {
    access_token: String,
}

#[derive(Clone)]
pub(super) struct SpotifyClient {
    pub(super) session: Arc<Mutex<session::Session>>,
    pub(super) http: Agent,
}

impl SpotifyClient {
    pub(super) fn new(http: Agent) -> ClientResult<Self> {
        let cache = config_path(SPOTIFY_SESSION_CACHE);
        let session = session::login(&http, "", &cache).or_else(|_| {
            let token = prompt_for_token(&http)?;
            session::login(&http, &token, &cache)
        })?;
        info!(
            username = %session.username,
            device_id = %session.device_id,
            spclient = %session.spclient,
            dealer = %session.dealer,
            "Authenticated Spotify session"
        );
        Ok(Self {
            session: Arc::new(Mutex::new(session)),
            http,
        })
    }

    pub(super) fn request(
        &self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        body: Vec<u8>,
    ) -> ClientResult<Vec<u8>> {
        let (token, endpoint, client_token) = self.with_session(|session| {
            Ok((
                session.authorization(&self.http)?.to_owned(),
                session.spclient.clone(),
                session.client_token.clone(),
            ))
        })?;
        let mut request = Request::builder()
            .method(method)
            .uri(format!("https://{endpoint}/{}", path.trim_start_matches('/')))
            .header("authorization", format!("Bearer {token}"))
            .header("client-token", client_token);
        for &(name, value) in headers {
            request = request.header(name, value);
        }
        Ok(self.http.run(request.body(body)?)?.body_mut().read_to_vec()?)
    }

    pub(super) fn request_proto<T: protobuf::Message>(
        &self,
        method: Method,
        path: &str,
        headers: &[(&str, &str)],
        message: &T,
    ) -> ClientResult<Vec<u8>> {
        self.request(method, path, headers, message.write_to_bytes()?)
    }

    pub(super) fn with_session<T>(
        &self,
        work: impl FnOnce(&mut session::Session) -> ClientResult<T>,
    ) -> ClientResult<T> {
        let mut session = self.session.lock();
        work(&mut session)
    }
}

/// Runs the interactive PKCE authorization flow in the user's browser.
fn prompt_for_token(http: &Agent) -> ClientResult<String> {
    let verifier = random_token::<32>()?;
    let expected_state = random_token::<16>()?;
    let challenge = URL_SAFE_NO_PAD.encode(digest(&SHA256, verifier.as_bytes()));
    let query = form_urlencoded::Serializer::new(String::new())
        .extend_pairs([
            ("client_id", CLIENT_ID),
            ("response_type", "code"),
            ("redirect_uri", REDIRECT_URI),
            ("code_challenge_method", "S256"),
            ("code_challenge", &challenge),
            ("state", &expected_state),
            ("scope", SCOPES),
        ])
        .finish();
    let url = format!("https://accounts.spotify.com/authorize?{query}");
    match Command::new("xdg-open").arg(&url).spawn() {
        Ok(_) => info!(%url, "Opened Spotify authorization URL in browser"),
        Err(err) => warn!(%err, %url, "Failed to open Spotify authorization URL; open it manually"),
    }

    let listener = TcpListener::bind(REDIRECT_ADDR)?;
    let (mut stream, _) = listener.accept()?;
    let mut buffer = [0; 1024];
    let count = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..count]);
    let query = request
        .split_whitespace()
        .nth(1)
        .and_then(|target| Some(target.split_once('?')?.1))
        .ok_or_else(|| client_error("invalid Spotify authorization response"))?;
    let params: HashMap<_, _> = form_urlencoded::parse(query.as_bytes()).collect();
    if params.get("state").is_none_or(|state| *state != expected_state) {
        return Err(client_error("Spotify authorization state did not match"));
    }
    let code = params
        .get("code")
        .ok_or_else(|| client_error("invalid Spotify authorization response"))?;

    let message = "Cantus connected successfully, this tab can be closed.";
    write!(
        stream,
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{message}",
        message.len(),
    )?;

    Ok(http
        .post(TOKEN_URL)
        .send_form([
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
            ("client_id", CLIENT_ID),
            ("code_verifier", &verifier),
        ])?
        .into_body()
        .read_json::<OAuthToken>()?
        .access_token)
}
