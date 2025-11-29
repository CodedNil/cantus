//! RSpotify is a wrapper for the [Spotify Web API][spotify-main], inspired by
//! [spotipy][spotipy-github]. It includes support for all the [authorization
//! flows][spotify-auth-flows], and helper methods for [all available
//! endpoints][spotify-reference].
//!
//! ## Configuration
//!
//! ### HTTP Client
//!
//! By default, RSpotify uses the [reqwest][reqwest-docs] asynchronous HTTP
//! client with its default TLS, but you can customize both the HTTP client and
//! the TLS with the following features:
//!
//! - [reqwest][reqwest-docs]: enabling
//!   `client-reqwest`, TLS available:
//!     + `reqwest-default-tls` (reqwest's default)
//!     + `reqwest-rustls-tls`
//!     + `reqwest-native-tls`
//!     + `reqwest-native-tls-vendored`
//! - [ureq][ureq-docs]: enabling `client-ureq`, TLS
//!   available:
//!     + `ureq-rustls-tls` (ureq's default)
//!     + `ureq-rustls-tls-native-certs` (`rustls` with OS root certificates)
//!
//! If you want to use a different client or TLS than the default ones, you'll
//! have to disable the default features and enable whichever you want. For
//! example, this would compile RSpotify with `reqwest` and the native TLS:
//!
//! ```toml
//! [dependencies]
//! rspotify = {
//!     version = "...",
//!     default-features = false,
//!     features = ["client-reqwest", "reqwest-native-tls"]
//! }
//! ```
//!
//! [`maybe_async`] internally enables RSpotify to  use both synchronous and
//! asynchronous HTTP clients. You can also use `ureq`, a synchronous client,
//! like so:
//!
//! ```toml
//! [dependencies]
//! rspotify = {
//!     version = "...",
//!     default-features = false,
//!     features = ["client-ureq", "ureq-rustls-tls"]
//! }
//! ```
//!
//! ### Proxies
//!
//! Both [reqwest][reqwest-proxies] and [ureq][ureq-proxying] support system
//! proxies by default. They both read `http_proxy`, `https_proxy`, `all_proxy`
//! and their uppercase variants `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`,
//! although the specific logic implementations are a little different.
//!
//! See also:
//! - [reqwest](https://docs.rs/reqwest/latest/src/reqwest/proxy.rs.html#897-920)
//! - [ureq](https://docs.rs/ureq/latest/src/ureq/proxy.rs.html#73-95)
//!
//! ### Environmental variables
//!
//! RSpotify supports the `dotenvy` crate, which allows you to save credentials
//! in a `.env` file. These will then be automatically available as
//! environmental values when using methods like [`Credentials::from_env`].
//!
//! ```toml
//! [dependencies]
//! rspotify = { version = "...", features = ["env-file"] }
//! ```
//!
//! ### CLI utilities
//!
//! RSpotify includes basic support for Cli apps to obtain access tokens by
//! prompting the user, after enabling the `cli` feature. See the
//! [Authorization](#authorization) section for more information.
//!
//! ## Getting Started
//!
//! ### Authorization
//!
//! All endpoints require app authorization; you will need to generate a token
//! that indicates that the client has been granted permission to perform
//! requests. You can start by [registering your app to get the necessary client
//! credentials][spotify-register-app]. Read the [official guide for a detailed
//! explanation of the different authorization flows
//! available][spotify-auth-flows].
//!
//! RSpotify has a different client for each of the available authentication
//! flows. They may implement the endpoints in
//! [`BaseClient`](crate::clients::BaseClient) or
//! [`OAuthClient`](crate::clients::OAuthClient) according to what kind of
//! flow it is. Please refer to their documentation for more details:
//!
//! * [Client Credentials Flow][spotify-client-creds]: see
//!   [`ClientCredsSpotify`].
//! * [Authorization Code Flow][spotify-auth-code]: see [`AuthCodeSpotify`].
//! * [Authorization Code Flow with Proof Key for Code Exchange
//!   (PKCE)][spotify-auth-code-pkce]: see [`AuthCodePkceSpotify`].
//! * [Implicit Grant Flow][spotify-implicit-grant]: unimplemented, as RSpotify
//!   has not been tested on a browser yet. If you'd like support for it, let us
//!   know in an issue!
//!
//! In order to help other developers to get used to `rspotify`, there are
//! public credentials available for a dummy account. You can test `rspotify`
//! with this account's `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET` inside
//! the [`.env` file](https://github.com/ramsayleung/rspotify/blob/master/.env)
//! for more details.
//!
//! ### WebAssembly
//!
//! RSpotify supports the `wasm32-unknown-unknown` target in combination
//! with the `client-reqwest` feature. HTTP requests must be processed async.
//! Other HTTP client configurations are not supported.
//!
//! [Spotify recommends][spotify-auth-flows] using [`AuthCodePkceSpotify`] for
//! authorization flows on the web.
//!
//! Importing the Client ID via `RSPOTIFY_CLIENT_ID` is not possible since WASM
//! web runtimes are isolated from the host environment. The client ID must be
//! passed explicitly to [`Credentials::new_pkce`]. Alternatively, it can be
//! embedded at compile time with the [`std::env!`] or
//! [`dotenv!`](https://crates.io/crates/dotenvy) macros.
//!
//! ### Examples
//!
//! There are some [available examples on the GitHub
//! repository][examples-github] which can serve as a learning tool.
//!
//! [spotipy-github]: https://github.com/plamere/spotipy
//! [reqwest-docs]: https://docs.rs/reqwest/
//! [reqwest-proxies]: https://docs.rs/reqwest/#proxies
//! [ureq-docs]: https://docs.rs/ureq/
//! [ureq-proxying]: https://docs.rs/ureq/latest/ureq/struct.Proxy.html
//! [examples-github]: https://github.com/ramsayleung/rspotify/tree/master/examples
//! [spotify-main]: https://developer.spotify.com/documentation/web-api/
//! [spotify-auth-flows]: https://developer.spotify.com/documentation/general/guides/authorization/
//! [spotify-reference]: https://developer.spotify.com/documentation/web-api/reference/
//! [spotify-register-app]: https://developer.spotify.com/dashboard/applications
//! [spotify-client-creds]: https://developer.spotify.com/documentation/general/guides/authorization/client-credentials/
//! [spotify-auth-code]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
//! [spotify-auth-code-pkce]: https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow
//! [spotify-implicit-grant]: https://developer.spotify.com/documentation/general/guides/authorization/implicit-grant

pub mod auth_code_pkce;
pub mod clients;
mod http;
pub mod model;
mod util;

use model::auth::Token;
use std::{collections::HashSet, fmt, net::SocketAddr, path::PathBuf, sync::Arc};
use thiserror::Error;

/// Common headers as constants.
pub mod params {
    pub const CLIENT_ID: &str = "client_id";
    pub const CODE: &str = "code";
    pub const GRANT_TYPE: &str = "grant_type";
    pub const GRANT_TYPE_AUTH_CODE: &str = "authorization_code";
    pub const GRANT_TYPE_REFRESH_TOKEN: &str = "refresh_token";
    pub const REDIRECT_URI: &str = "redirect_uri";
    pub const REFRESH_TOKEN: &str = "refresh_token";
    pub const RESPONSE_TYPE_CODE: &str = "code";
    pub const RESPONSE_TYPE: &str = "response_type";
    pub const SCOPE: &str = "scope";
    pub const STATE: &str = "state";
    pub const CODE_CHALLENGE: &str = "code_challenge";
    pub const CODE_VERIFIER: &str = "code_verifier";
    pub const CODE_CHALLENGE_METHOD: &str = "code_challenge_method";
    pub const CODE_CHALLENGE_METHOD_S256: &str = "S256";
}

/// Common alphabets for random number generation and similars
pub mod alphabets {
    pub const ALPHANUM: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    /// From <https://datatracker.ietf.org/doc/html/rfc7636#section-4.1>
    pub const PKCE_CODE_VERIFIER: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~";
}

pub mod auth_urls {
    pub const AUTHORIZE: &str = "authorize";
    pub const TOKEN: &str = "api/token";
}

/// Possible errors returned from the `rspotify` client.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),

    // Note that this type is boxed because its size might be very large in
    // comparison to the rest. For more information visit:
    // https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    #[error("http error: {0}")]
    Http(Box<ureq::Error>),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),

    #[error("cli error: {0}")]
    Cli(String),

    #[error("token callback function error: {0}")]
    TokenCallbackFn(#[from] CallbackError),

    #[error("model error: {0}")]
    Model(#[from] model::error::ModelError),

    #[error("Token is not valid")]
    InvalidToken,

    #[error("Failed to bind server to {addr} ({e})")]
    AuthCodeListenerBind { addr: SocketAddr, e: std::io::Error },

    #[error("Listener terminated without accepting a connection")]
    AuthCodeListenerTerminated,

    #[error("Failed to read redirect URI from HTTP request")]
    AuthCodeListenerRead,

    #[error("Failed to parse redirect URI {0} from HTTP request")]
    AuthCodeListenerParse(String),

    #[error("Failed to write HTTP response")]
    AuthCodeListenerWrite,
}

// The conversion has to be done manually because it's in a `Box<T>`
impl From<ureq::Error> for ClientError {
    fn from(err: ureq::Error) -> Self {
        Self::Http(Box::new(err))
    }
}

pub type ClientResult<T> = Result<T, ClientError>;

pub const DEFAULT_API_BASE_URL: &str = "https://api.spotify.com/v1/";
pub const DEFAULT_AUTH_BASE_URL: &str = "https://accounts.spotify.com/";
pub const DEFAULT_CACHE_PATH: &str = ".spotify_token_cache.json";

#[derive(Error, Debug)]
pub enum CallbackError {}

/// A callback function is invokved whenever successfully request or refetch a new token.
pub struct TokenCallback(pub Box<dyn Fn(Token) -> Result<(), CallbackError> + Send + Sync>);

impl fmt::Debug for TokenCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TokenCallback")
    }
}

/// Struct to configure the Spotify client.
#[derive(Debug, Clone)]
pub struct Config {
    /// The Spotify API prefix, [`DEFAULT_API_BASE_URL`] by default.
    pub api_base_url: String,

    /// The Spotify Authentication prefix, [`DEFAULT_AUTH_BASE_URL`] by default.
    pub auth_base_url: String,

    /// The cache file path, in case it's used. By default it's
    /// [`DEFAULT_CACHE_PATH`]
    pub cache_path: PathBuf,

    /// Whether or not to save the authentication token into a JSON file,
    /// then reread the token from JSON file when launching the program without
    /// following the full auth process again
    pub token_cached: bool,

    /// Whether or not to check if the token has expired when sending a
    /// request with credentials, and in that case, automatically refresh it.
    pub token_refreshing: bool,

    /// Whenever client succeeds to request or refresh a token, the callback function
    /// will be invoked
    pub token_callback_fn: Arc<Option<TokenCallback>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_base_url: String::from(DEFAULT_API_BASE_URL),
            auth_base_url: String::from(DEFAULT_AUTH_BASE_URL),
            cache_path: PathBuf::from(DEFAULT_CACHE_PATH),
            token_cached: false,
            token_refreshing: true,
            token_callback_fn: Arc::new(None),
        }
    }
}

/// Generate `length` random chars from the Operating System.
///
/// It is assumed that system always provides high-quality cryptographically
/// secure random data, ideally backed by hardware entropy sources.
pub fn generate_random_string(length: usize, alphabet: &[u8]) -> String {
    let mut buf = vec![0u8; length];
    getrandom::fill(&mut buf).unwrap();
    let range = alphabet.len();

    buf.iter()
        .map(|byte| alphabet[*byte as usize % range] as char)
        .collect()
}

pub fn join_scopes(scopes: &HashSet<String>) -> String {
    scopes
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Simple client credentials object for Spotify.
#[derive(Debug, Clone, Default)]
pub struct Credentials {
    pub id: String,
}

/// Structure that holds the required information for requests with OAuth.
#[derive(Debug, Clone)]
pub struct OAuth {
    pub redirect_uri: String,
    /// The state is generated by default, as suggested by the OAuth2 spec:
    /// [Cross-Site Request Forgery](https://tools.ietf.org/html/rfc6749#section-10.12)
    pub state: String,
    /// You could use macro [scopes!](crate::scopes) to build it at compile time easily
    pub scopes: HashSet<String>,
}

impl Default for OAuth {
    fn default() -> Self {
        Self {
            redirect_uri: String::new(),
            state: generate_random_string(16, alphabets::ALPHANUM),
            scopes: HashSet::new(),
        }
    }
}
