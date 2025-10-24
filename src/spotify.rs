use anyhow::Result;
use parking_lot::Mutex;
use rspotify::{
    AuthCodeSpotify, Config, Credentials, OAuth,
    model::{CurrentUserQueue, PlayableItem},
    prelude::OAuthClient,
    scopes,
};
use std::sync::{Arc, LazyLock};

pub static CURRENT_SONGS: LazyLock<Arc<Mutex<Option<CurrentUserQueue>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub async fn init() -> Result<()> {
    let spotify = AuthCodeSpotify::with_config(
        Credentials::from_env()
            .expect("Missing env credentials RSPOTIFY_CLIENT_ID RSPOTIFY_CLIENT_SECRET"),
        OAuth {
            redirect_uri: String::from("http://127.0.0.1:7474/callback"),
            scopes: scopes!(
                "user-read-playback-state",
                "user-modify-playback-state",
                "user-read-currently-playing",
                "playlist-read-private",
                "playlist-read-collaborative",
                "playlist-modify-private",
                "playlist-modify-public",
                "user-read-playback-position",
                "user-read-recently-played"
            ),
            ..Default::default()
        },
        Config {
            token_cached: true,
            ..Default::default()
        },
    );

    // Get authorization token
    let url = spotify.get_authorize_url(true).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    // Running the requests
    if let Ok(Some(playing)) = spotify.current_user_playing_item().await {
        println!("Progress: {:?}", playing.progress);
        if let Some(PlayableItem::Track(item)) = playing.item {
            println!(
                "Item: {:?}\n{:?}\n{:?}\n{:?}\n{:?}",
                item.name,
                item.album.name,
                item.artists.first().map(|a| a.name.clone()),
                item.album.images.last().map(|i| i.url.clone()),
                item.duration,
            );
        }
    }
    if let Ok(queue) = spotify.current_user_queue().await {
        println!("Queue: {:?}", queue.queue.len());
        let mut binding = CURRENT_SONGS.lock();
        *binding = Some(queue);
    }

    Ok(())
}
