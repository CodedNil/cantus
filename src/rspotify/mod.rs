pub mod client;
pub mod custom_serde;
pub mod model;

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

    #[error("model error: {0}")]
    Model(#[from] model::ModelError),

    #[error("Token is not valid")]
    InvalidToken,
}

// The conversion has to be done manually because it's in a `Box<T>`
impl From<ureq::Error> for ClientError {
    fn from(err: ureq::Error) -> Self {
        Self::Http(Box::new(err))
    }
}

pub type ClientResult<T> = Result<T, ClientError>;

/// Generate `length` random chars
pub fn generate_random_string(length: usize, alphabet: &[u8]) -> String {
    let range = alphabet.len();
    (0..length)
        .map(|_| alphabet[fastrand::usize(..range)] as char)
        .collect()
}
