pub mod auth;
pub mod client;
pub mod custom_serde;
pub mod model;

use auth::Token;
use std::{collections::HashSet, fmt, net::SocketAddr, path::PathBuf, sync::Arc};
use thiserror::Error;

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
    Model(#[from] model::ModelError),

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
    /// The cache file path, in case it's used. By default it's [`DEFAULT_CACHE_PATH`]
    pub cache_path: PathBuf,

    /// Whenever client succeeds to request or refresh a token, the callback function will be invoked
    pub token_callback_fn: Arc<Option<TokenCallback>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_path: PathBuf::from(DEFAULT_CACHE_PATH),
            token_callback_fn: Arc::new(None),
        }
    }
}

/// Generate `length` random chars from the Operating System.
///
/// It is assumed that system always provides high-quality cryptographically
/// secure random data, ideally backed by hardware entropy sources.
pub fn generate_random_string(length: usize, alphabet: &[u8]) -> String {
    let range = alphabet.len();
    (0..length)
        .map(|_| alphabet[fastrand::usize(..range)] as char)
        .collect()
}

/// Structure that holds the required information for requests with OAuth.
#[derive(Debug, Clone)]
pub struct OAuth {
    pub redirect_uri: String,
    pub state: String,
    pub scopes: HashSet<String>,
}

impl Default for OAuth {
    fn default() -> Self {
        Self {
            redirect_uri: String::new(),
            state: generate_random_string(
                16,
                b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            ),
            scopes: HashSet::new(),
        }
    }
}
