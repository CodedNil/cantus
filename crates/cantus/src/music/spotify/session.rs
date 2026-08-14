use super::{CLIENT_ID, ClientResult, read_cache, write_cache};
use hmac::{Hmac, Mac};
use librespot_protocol::{
    authentication::{APWelcome, AuthenticationType, ClientResponseEncrypted, CpuFamily, Os},
    client_info::ClientInfo,
    clienttoken_http::{
        ClientTokenRequest, ClientTokenRequestType, ClientTokenResponse, ClientTokenResponseType,
    },
    credentials::StoredCredential,
    hashcash::{HashcashChallenge, HashcashSolution},
    keyexchange::{
        APLoginFailed, APResponseMessage, ClientHello, ClientResponsePlaintext, Cryptosuite, Platform,
        Product,
    },
    login5::{
        ChallengeSolution, ChallengeSolutions, LoginOk, LoginRequest, LoginResponse, login_request,
        login_response,
    },
};
use num_bigint::BigUint;
use protobuf::{Message as _, MessageField, well_known_types::duration::Duration as ProtoDuration};
use ring::signature::{RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY, RsaPublicKeyComponents};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use shannon::Shannon;
use std::{
    env::consts::ARCH,
    fmt::Write as _,
    io::{self, Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use tracing::warn;
use ureq::Agent;

const AP_RESOLVE: &str = "https://apresolve.spotify.com/?type=accesspoint&type=spclient&type=dealer";
const CLIENT_TOKEN_URL: &str = "https://clienttoken.spotify.com/v1/clienttoken";
const LOGIN5_URL: &str = "https://login5.spotify.com/v3/login";
const SPOTIFY_VERSION: u64 = 127_700_358;

#[derive(Serialize, Deserialize)]
struct CachedSession {
    device_id: String,
    #[serde(flatten)]
    credentials: Credentials,
}

pub struct Session {
    pub access_token: String,
    pub client_token: String,
    pub device_id: String,
    pub spclient: String,
    pub dealer: String,
    pub username: String,
    credentials: Credentials,
    cache_path: PathBuf,
    expires_at: Instant,
}

impl Session {
    pub fn authorization(&mut self, http: &Agent) -> ClientResult<&str> {
        if Instant::now() + Duration::from_secs(10) >= self.expires_at {
            let login = login5(http, &self.device_id, &self.client_token, &self.credentials)?;
            self.access_token = login.access_token;
            self.expires_at =
                Instant::now() + Duration::from_secs(login.access_token_expires_in.max(1) as u64);
            if !login.stored_credential.is_empty() {
                self.credentials = Credentials {
                    username: login.username,
                    data: login.stored_credential,
                };
                if let Err(error) = write_cache(
                    &self.cache_path,
                    &CachedSession {
                        device_id: self.device_id.clone(),
                        credentials: self.credentials.clone(),
                    },
                ) {
                    warn!(%error, "Failed to persist refreshed Spotify credentials");
                }
            }
        }
        Ok(&self.access_token)
    }
}

pub fn login(http: &Agent, oauth_token: &str, cache_path: &Path) -> ClientResult<Session> {
    let cached = read_cache::<CachedSession>(cache_path);
    if cached.is_none() && oauth_token.is_empty() {
        return Err(io::Error::other("Spotify authentication required").into());
    }
    let device_id = cached
        .as_ref()
        .map_or_else(device_id, |cache| cache.device_id.clone());
    let client_token = client_token(http, &device_id)?;
    let endpoints = http
        .get(AP_RESOLVE)
        .call()?
        .body_mut()
        .read_json::<AccessPoints>()?;
    let credentials = if let Some(cache) = cached {
        let cached = ap_login(
            &endpoints.accesspoint,
            &device_id,
            AuthenticationType::AUTHENTICATION_STORED_SPOTIFY_CREDENTIALS,
            Some(cache.credentials.username.clone()),
            cache.credentials.data,
        );
        if oauth_token.is_empty() {
            cached?
        } else {
            cached
                .inspect_err(|error| warn!(%error, "Cached Spotify credentials failed; retrying OAuth"))
                .or_else(|_| {
                    ap_login(
                        &endpoints.accesspoint,
                        &device_id,
                        AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN,
                        None,
                        oauth_token.as_bytes().to_vec(),
                    )
                })?
        }
    } else {
        ap_login(
            &endpoints.accesspoint,
            &device_id,
            AuthenticationType::AUTHENTICATION_SPOTIFY_TOKEN,
            None,
            oauth_token.as_bytes().to_vec(),
        )?
    };
    write_cache(
        cache_path,
        &CachedSession {
            device_id: device_id.clone(),
            credentials: credentials.clone(),
        },
    )?;
    let login = login5(http, &device_id, &client_token, &credentials)?;
    let expires_at = Instant::now() + Duration::from_secs(login.access_token_expires_in.max(1) as u64);
    let credentials = if login.stored_credential.is_empty() {
        credentials
    } else {
        Credentials {
            username: login.username.clone(),
            data: login.stored_credential,
        }
    };
    Ok(Session {
        access_token: login.access_token,
        client_token,
        device_id,
        spclient: first(endpoints.spclient, "spclient")?,
        dealer: first(endpoints.dealer, "dealer")?,
        username: credentials.username.clone(),
        credentials,
        cache_path: cache_path.to_owned(),
        expires_at,
    })
}

#[derive(Deserialize)]
struct AccessPoints {
    accesspoint: Vec<String>,
    spclient: Vec<String>,
    dealer: Vec<String>,
}

fn first(endpoints: Vec<String>, kind: &str) -> ClientResult<String> {
    endpoints
        .into_iter()
        .next()
        .ok_or_else(|| io::Error::other(format!("Spotify returned no {kind} endpoint")).into())
}

#[derive(Clone, Serialize, Deserialize)]
struct Credentials {
    username: String,
    data: Vec<u8>,
}

fn device_id() -> String {
    let mut bytes = [0; 20];
    getrandom::fill(&mut bytes).expect("operating-system randomness unavailable");
    bytes.iter().fold(String::with_capacity(40), |mut id, byte| {
        let _ = write!(id, "{byte:02x}");
        id
    })
}

fn client_token(http: &Agent, device_id: &str) -> ClientResult<String> {
    let mut request = ClientTokenRequest::new();
    request.request_type = ClientTokenRequestType::REQUEST_CLIENT_DATA_REQUEST.into();
    let data = request.mut_client_data();
    data.client_version = env!("CARGO_PKG_VERSION").into();
    data.client_id = CLIENT_ID.into();
    let connectivity = data.mut_connectivity_sdk_data();
    connectivity.device_id = device_id.into();
    connectivity
        .platform_specific_data
        .mut_or_insert_default()
        .mut_desktop_linux()
        .hardware = ARCH.into();
    let response: ClientTokenResponse = post_proto(http, CLIENT_TOKEN_URL, None, &request)?;
    if response.response_type.enum_value_or_default()
        == ClientTokenResponseType::RESPONSE_GRANTED_TOKEN_RESPONSE
    {
        Ok(response.granted_token().token.clone())
    } else {
        Err(io::Error::other("Spotify client-token challenge unsupported").into())
    }
}

fn login5(
    http: &Agent,
    device_id: &str,
    client_token: &str,
    credentials: &Credentials,
) -> ClientResult<LoginOk> {
    let mut request = LoginRequest {
        client_info: MessageField::some(ClientInfo {
            client_id: CLIENT_ID.into(),
            device_id: device_id.into(),
            ..Default::default()
        }),
        login_method: Some(login_request::Login_method::StoredCredential(StoredCredential {
            username: credentials.username.clone(),
            data: credentials.data.clone(),
            ..Default::default()
        })),
        ..Default::default()
    };
    let mut response: LoginResponse = post_proto(http, LOGIN5_URL, Some(client_token), &request)?;
    if let Some(login_response::Response::Challenges(challenges)) = response.response {
        let started = Instant::now();
        request.login_context = response.login_context;
        request.challenge_solutions = MessageField::some(ChallengeSolutions {
            solutions: challenges
                .challenges
                .into_iter()
                .filter_map(|challenge| {
                    challenge.has_hashcash().then(|| {
                        let mut solution = ChallengeSolution::new();
                        solution.set_hashcash(solve_hashcash(
                            &request.login_context,
                            challenge.hashcash(),
                            started,
                        ));
                        solution
                    })
                })
                .collect(),
            ..Default::default()
        });
        response = post_proto(http, LOGIN5_URL, Some(client_token), &request)?;
    }
    match response.response {
        Some(login_response::Response::Ok(ok)) => Ok(ok),
        Some(login_response::Response::Error(error)) => {
            Err(io::Error::other(format!("Spotify Login5 rejected credentials ({error:?})")).into())
        }
        _ => Err(io::Error::other("Spotify Login5 returned no token").into()),
    }
}

fn post_proto<T: protobuf::Message, R: protobuf::Message>(
    http: &Agent,
    url: &str,
    client_token: Option<&str>,
    request: &T,
) -> ClientResult<R> {
    let mut builder = http
        .post(url)
        .header("accept", "application/x-protobuf")
        .header("content-type", "application/x-protobuf");
    if let Some(token) = client_token {
        builder = builder.header("client-token", token);
    }
    let bytes = builder
        .send(request.write_to_bytes()?)?
        .body_mut()
        .read_to_vec()?;
    R::parse_from_bytes(&bytes).map_err(Into::into)
}

fn solve_hashcash(context: &[u8], challenge: &HashcashChallenge, started: Instant) -> HashcashSolution {
    let context_hash = Sha1::digest(context);
    let mut suffix = [0; 16];
    suffix[..8].copy_from_slice(&context_hash[12..]);
    loop {
        let hash = Sha1::new()
            .chain_update(&challenge.prefix)
            .chain_update(suffix)
            .finalize();
        if trailing_zeros(&hash) >= challenge.length as u32 {
            let elapsed = started.elapsed();
            return HashcashSolution {
                suffix: suffix.to_vec(),
                duration: MessageField::some(ProtoDuration {
                    seconds: elapsed.as_secs() as i64,
                    nanos: elapsed.subsec_nanos() as i32,
                    ..Default::default()
                }),
                ..Default::default()
            };
        }
        increment(&mut suffix[..8]);
        increment(&mut suffix[8..]);
    }
}

fn trailing_zeros(hash: &[u8]) -> u32 {
    let mut total = 0;
    for byte in hash.iter().rev() {
        let zeros = byte.trailing_zeros();
        total += zeros;
        if zeros != 8 {
            break;
        }
    }
    total
}

fn increment(counter: &mut [u8]) {
    for byte in counter.iter_mut().rev() {
        *byte = byte.wrapping_add(1);
        if *byte != 0 {
            break;
        }
    }
}

fn ap_login(
    endpoints: &[String],
    device_id: &str,
    kind: AuthenticationType,
    username: Option<String>,
    data: Vec<u8>,
) -> ClientResult<Credentials> {
    let mut errors = Vec::new();
    let mut stream = endpoints
        .iter()
        .find_map(|endpoint| match TcpStream::connect(endpoint) {
            Ok(stream) => Some(stream),
            Err(error) => {
                errors.push(format!("{endpoint}: {error}"));
                None
            }
        })
        .ok_or_else(|| {
            io::Error::other(format!(
                "Spotify access points unavailable: {}",
                errors.join(", ")
            ))
        })?;
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;
    let mut private = [0; 95];
    getrandom::fill(&mut private)?;
    let private = BigUint::from_bytes_le(&private);
    let public = BigUint::from(2u8).modpow(&private, &dh_prime()).to_bytes_be();
    let mut hello = ClientHello::new();
    let build = hello.build_info.mut_or_insert_default();
    build.set_product(Product::PRODUCT_CLIENT);
    build.set_platform(platform());
    build.set_version(SPOTIFY_VERSION);
    hello
        .cryptosuites_supported
        .push(Cryptosuite::CRYPTO_SUITE_SHANNON.into());
    let crypto = hello
        .login_crypto_hello
        .mut_or_insert_default()
        .diffie_hellman
        .mut_or_insert_default();
    crypto.set_gc(public);
    crypto.set_server_keys_known(1);
    hello.set_client_nonce(random::<16>()?.to_vec());
    hello.set_padding(vec![0x1e]);
    let mut transcript = vec![0, 4];
    let body = hello.write_to_bytes()?;
    transcript.extend_from_slice(&(body.len() as u32 + 6).to_be_bytes());
    transcript.extend_from_slice(&body);
    stream.write_all(&transcript)?;
    let response = read_plain(&mut stream, &mut transcript)?;
    let response = APResponseMessage::parse_from_bytes(&response)?;
    let challenge = response
        .challenge
        .get_or_default()
        .login_crypto_challenge
        .get_or_default()
        .diffie_hellman
        .get_or_default();
    verify_server_key(challenge.gs(), challenge.gs_signature())?;
    let shared = BigUint::from_bytes_be(challenge.gs())
        .modpow(&private, &dh_prime())
        .to_bytes_be();
    let (proof, send_key, receive_key) = session_keys(&shared, &transcript)?;
    let mut response = ClientResponsePlaintext::new();
    response
        .login_crypto_response
        .mut_or_insert_default()
        .diffie_hellman
        .mut_or_insert_default()
        .set_hmac(proof);
    response.pow_response.mut_or_insert_default();
    response.crypto_response.mut_or_insert_default();
    let response = response.write_to_bytes()?;
    stream.write_all(&(response.len() as u32 + 4).to_be_bytes())?;
    stream.write_all(&response)?;
    let mut transport = ApTransport::new(stream, &send_key, &receive_key);
    transport.send(
        0xab,
        &authentication_packet(device_id, kind, username, data).write_to_bytes()?,
    )?;
    let (command, response) = transport.receive()?;
    match command {
        0xac => {
            let welcome = APWelcome::parse_from_bytes(&response)?;
            Ok(Credentials {
                username: welcome.canonical_username().to_owned(),
                data: welcome.reusable_auth_credentials().to_owned(),
            })
        }
        0xad => {
            let failure = APLoginFailed::parse_from_bytes(&response)?;
            Err(io::Error::other(format!("Spotify AP login failed ({:?})", failure.error_code())).into())
        }
        _ => Err(io::Error::other(format!("unexpected Spotify AP packet {command:#x}")).into()),
    }
}

fn authentication_packet(
    device_id: &str,
    kind: AuthenticationType,
    username: Option<String>,
    data: Vec<u8>,
) -> ClientResponseEncrypted {
    let mut packet = ClientResponseEncrypted::new();
    if let Some(username) = username {
        packet
            .login_credentials
            .mut_or_insert_default()
            .set_username(username);
    }
    let login = packet.login_credentials.mut_or_insert_default();
    login.set_typ(kind);
    login.set_auth_data(data);
    let system = packet.system_info.mut_or_insert_default();
    system.set_cpu_family(cpu());
    system.set_os(Os::OS_LINUX);
    system.set_system_information_string(format!("Cantus/{}", env!("CARGO_PKG_VERSION")));
    system.set_device_id(device_id.into());
    packet.set_version_string(format!("Cantus {}", env!("CARGO_PKG_VERSION")));
    packet
}

fn read_plain(stream: &mut TcpStream, transcript: &mut Vec<u8>) -> io::Result<Vec<u8>> {
    let mut size = [0; 4];
    stream.read_exact(&mut size)?;
    transcript.extend_from_slice(&size);
    let mut body = vec![0; u32::from_be_bytes(size) as usize - 4];
    stream.read_exact(&mut body)?;
    transcript.extend_from_slice(&body);
    Ok(body)
}

fn session_keys(secret: &[u8], transcript: &[u8]) -> ClientResult<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let mut keys = Vec::with_capacity(100);
    for index in 1..=5 {
        let mut hmac = Hmac::<Sha1>::new_from_slice(secret)?;
        hmac.update(transcript);
        hmac.update(&[index]);
        keys.extend_from_slice(&hmac.finalize().into_bytes());
    }
    let mut proof = Hmac::<Sha1>::new_from_slice(&keys[..20])?;
    proof.update(transcript);
    Ok((
        proof.finalize().into_bytes().to_vec(),
        keys[20..52].to_vec(),
        keys[52..84].to_vec(),
    ))
}

struct ApTransport {
    stream: TcpStream,
    send: Shannon,
    receive: Shannon,
    send_nonce: u32,
    receive_nonce: u32,
}

impl ApTransport {
    fn new(stream: TcpStream, send_key: &[u8], receive_key: &[u8]) -> Self {
        Self {
            stream,
            send: Shannon::new(send_key),
            receive: Shannon::new(receive_key),
            send_nonce: 0,
            receive_nonce: 0,
        }
    }

    fn send(&mut self, command: u8, payload: &[u8]) -> io::Result<()> {
        let mut packet = Vec::with_capacity(payload.len() + 7);
        packet.push(command);
        packet.extend_from_slice(&(payload.len() as u16).to_be_bytes());
        packet.extend_from_slice(payload);
        self.send.nonce_u32(self.send_nonce);
        self.send_nonce += 1;
        self.send.encrypt(&mut packet);
        let mut mac = [0; 4];
        self.send.finish(&mut mac);
        packet.extend_from_slice(&mac);
        self.stream.write_all(&packet)
    }

    fn receive(&mut self) -> io::Result<(u8, Vec<u8>)> {
        let mut header = [0; 3];
        self.stream.read_exact(&mut header)?;
        self.receive.nonce_u32(self.receive_nonce);
        self.receive_nonce += 1;
        self.receive.decrypt(&mut header);
        let size = u16::from_be_bytes([header[1], header[2]]) as usize;
        let mut payload = vec![0; size];
        let mut mac = [0; 4];
        self.stream.read_exact(&mut payload)?;
        self.stream.read_exact(&mut mac)?;
        self.receive.decrypt(&mut payload);
        self.receive.check_mac(&mac)?;
        Ok((header[0], payload))
    }
}

fn random<const N: usize>() -> Result<[u8; N], getrandom::Error> {
    let mut bytes = [0; N];
    getrandom::fill(&mut bytes)?;
    Ok(bytes)
}

fn platform() -> Platform {
    match ARCH {
        "x86_64" => Platform::PLATFORM_LINUX_X86_64,
        "arm" | "aarch64" => Platform::PLATFORM_LINUX_ARM,
        _ => Platform::PLATFORM_LINUX_X86,
    }
}

fn cpu() -> CpuFamily {
    match ARCH {
        "x86_64" => CpuFamily::CPU_X86_64,
        "x86" => CpuFamily::CPU_X86,
        "arm" | "aarch64" => CpuFamily::CPU_ARM,
        _ => CpuFamily::CPU_UNKNOWN,
    }
}

fn dh_prime() -> BigUint {
    BigUint::from_bytes_be(&[
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc9, 0x0f, 0xda, 0xa2, 0x21, 0x68, 0xc2, 0x34,
        0xc4, 0xc6, 0x62, 0x8b, 0x80, 0xdc, 0x1c, 0xd1, 0x29, 0x02, 0x4e, 0x08, 0x8a, 0x67, 0xcc, 0x74,
        0x02, 0x0b, 0xbe, 0xa6, 0x3b, 0x13, 0x9b, 0x22, 0x51, 0x4a, 0x08, 0x79, 0x8e, 0x34, 0x04, 0xdd,
        0xef, 0x95, 0x19, 0xb3, 0xcd, 0x3a, 0x43, 0x1b, 0x30, 0x2b, 0x0a, 0x6d, 0xf2, 0x5f, 0x14, 0x37,
        0x4f, 0xe1, 0x35, 0x6d, 0x6d, 0x51, 0xc2, 0x45, 0xe4, 0x85, 0xb5, 0x76, 0x62, 0x5e, 0x7e, 0xc6,
        0xf4, 0x4c, 0x42, 0xe9, 0xa6, 0x3a, 0x36, 0x20, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    ])
}

fn verify_server_key(key: &[u8], signature: &[u8]) -> ClientResult<()> {
    RsaPublicKeyComponents {
        n: SERVER_KEY.as_slice(),
        e: [1, 0, 1].as_slice(),
    }
    .verify(&RSA_PKCS1_2048_8192_SHA1_FOR_LEGACY_USE_ONLY, key, signature)
    .map_err(|_| io::Error::other("Spotify server key verification failed").into())
}

const SERVER_KEY: [u8; 256] = [
    0xac, 0xe0, 0x46, 0x0b, 0xff, 0xc2, 0x30, 0xaf, 0xf4, 0x6b, 0xfe, 0xc3, 0xbf, 0xbf, 0x86, 0x3d,
    0xa1, 0x91, 0xc6, 0xcc, 0x33, 0x6c, 0x93, 0xa1, 0x4f, 0xb3, 0xb0, 0x16, 0x12, 0xac, 0xac, 0x6a,
    0xf1, 0x80, 0xe7, 0xf6, 0x14, 0xd9, 0x42, 0x9d, 0xbe, 0x2e, 0x34, 0x66, 0x43, 0xe3, 0x62, 0xd2,
    0x32, 0x7a, 0x1a, 0x0d, 0x92, 0x3b, 0xae, 0xdd, 0x14, 0x02, 0xb1, 0x81, 0x55, 0x05, 0x61, 0x04,
    0xd5, 0x2c, 0x96, 0xa4, 0x4c, 0x1e, 0xcc, 0x02, 0x4a, 0xd4, 0xb2, 0x0c, 0x00, 0x1f, 0x17, 0xed,
    0xc2, 0x2f, 0xc4, 0x35, 0x21, 0xc8, 0xf0, 0xcb, 0xae, 0xd2, 0xad, 0xd7, 0x2b, 0x0f, 0x9d, 0xb3,
    0xc5, 0x32, 0x1a, 0x2a, 0xfe, 0x59, 0xf3, 0x5a, 0x0d, 0xac, 0x68, 0xf1, 0xfa, 0x62, 0x1e, 0xfb,
    0x2c, 0x8d, 0x0c, 0xb7, 0x39, 0x2d, 0x92, 0x47, 0xe3, 0xd7, 0x35, 0x1a, 0x6d, 0xbd, 0x24, 0xc2,
    0xae, 0x25, 0x5b, 0x88, 0xff, 0xab, 0x73, 0x29, 0x8a, 0x0b, 0xcc, 0xcd, 0x0c, 0x58, 0x67, 0x31,
    0x89, 0xe8, 0xbd, 0x34, 0x80, 0x78, 0x4a, 0x5f, 0xc9, 0x6b, 0x89, 0x9d, 0x95, 0x6b, 0xfc, 0x86,
    0xd7, 0x4f, 0x33, 0xa6, 0x78, 0x17, 0x96, 0xc9, 0xc3, 0x2d, 0x0d, 0x32, 0xa5, 0xab, 0xcd, 0x05,
    0x27, 0xe2, 0xf7, 0x10, 0xa3, 0x96, 0x13, 0xc4, 0x2f, 0x99, 0xc0, 0x27, 0xbf, 0xed, 0x04, 0x9c,
    0x3c, 0x27, 0x58, 0x04, 0xb6, 0xb2, 0x19, 0xf9, 0xc1, 0x2f, 0x02, 0xe9, 0x48, 0x63, 0xec, 0xa1,
    0xb6, 0x42, 0xa0, 0x9d, 0x48, 0x25, 0xf8, 0xb3, 0x9d, 0xd0, 0xe8, 0x6a, 0xf9, 0x48, 0x4d, 0xa1,
    0xc2, 0xba, 0x86, 0x30, 0x42, 0xea, 0x9d, 0xb3, 0x08, 0x6c, 0x19, 0x0e, 0x48, 0xb3, 0x9d, 0x66,
    0xeb, 0x00, 0x06, 0xa2, 0x5a, 0xee, 0xa1, 0x1b, 0x13, 0x87, 0x3c, 0xd7, 0x19, 0xe6, 0x55, 0xbd,
];
