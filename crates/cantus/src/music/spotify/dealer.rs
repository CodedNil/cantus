use super::{ClientResult, SpotifyClient, WorkerEvent};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use flate2::read::GzDecoder;
use librespot_protocol::connect::ClusterUpdate;
use protobuf::Message as _;
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    io::{self, Read},
    str,
    sync::mpsc::Sender,
    thread,
    time::Duration,
};
use tracing::{info, warn};
use tungstenite::Message;

#[derive(Deserialize)]
struct Envelope {
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    payloads: Vec<Value>,
    #[serde(default)]
    uri: String,
}

pub fn connect(client: SpotifyClient, sender: Sender<WorkerEvent>) {
    thread::spawn(move || {
        let mut retry_delay = Duration::from_secs(1);
        loop {
            if let Err(error) = run(&sender, &client) {
                warn!(%error, "Spotify Dealer disconnected; reconnecting");
            }
            thread::sleep(retry_delay);
            retry_delay = (retry_delay * 2).min(Duration::from_secs(30));
        }
    });
}

fn run(sender: &Sender<WorkerEvent>, client: &SpotifyClient) -> ClientResult<()> {
    let (endpoint, token) = client.with_session(|session| {
        Ok((
            session.dealer.clone(),
            session.authorization(&client.http)?.to_owned(),
        ))
    })?;
    let url = format!("wss://{endpoint}/?access_token={token}");
    let (mut socket, _) = tungstenite::connect(url)?;
    info!(%endpoint, "Connected Spotify Dealer");

    loop {
        match socket.read()? {
            Message::Text(json) => handle(sender, &json)?,
            Message::Binary(bytes) => handle(sender, str::from_utf8(&bytes)?)?,
            Message::Close(_) => return Err(io::Error::other("Dealer closed connection").into()),
            Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => {}
        }
    }
}

fn handle(sender: &Sender<WorkerEvent>, json: &str) -> ClientResult<()> {
    let envelope: Envelope = serde_json::from_str(json)?;
    if envelope.uri.starts_with("hm://pusher/v1/connections/") {
        if let Some(connection_id) = header(&envelope.headers, "Spotify-Connection-Id") {
            sender.send(WorkerEvent::Connected(connection_id.to_owned()))?;
        }
        return Ok(());
    }
    if envelope.uri.starts_with("hm://playlist/v2/") {
        sender.send(WorkerEvent::PlaylistsChanged)?;
        return Ok(());
    }
    if envelope.uri != "hm://connect-state/v1/cluster" {
        return Ok(());
    }
    let Some(payload) = envelope.payloads.into_iter().next() else {
        return Ok(());
    };
    let mut bytes = match payload {
        Value::String(payload) => STANDARD.decode(payload)?,
        Value::Array(_) => serde_json::from_value(payload)?,
        _ => return Ok(()),
    };
    if header(&envelope.headers, "Transfer-Encoding")
        .is_some_and(|encoding| encoding.eq_ignore_ascii_case("gzip"))
    {
        let mut decoded = Vec::new();
        GzDecoder::new(bytes.as_slice()).read_to_end(&mut decoded)?;
        bytes = decoded;
    }
    let update = ClusterUpdate::parse_from_bytes(&bytes)?;
    sender.send(WorkerEvent::Cluster(
        update.cluster.into_option().unwrap_or_default(),
    ))?;
    Ok(())
}

fn header<'a>(headers: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find_map(|(key, value)| key.eq_ignore_ascii_case(name).then_some(value.as_str()))
}
