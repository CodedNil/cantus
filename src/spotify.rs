use anyhow::Result;
use mpris::{PlaybackStatus, PlayerFinder};
use parking_lot::Mutex;
use rspotify::{
    AuthCodeSpotify, Config, Credentials, OAuth,
    model::{AdditionalType, CurrentUserQueue},
    prelude::OAuthClient,
    scopes,
};
use std::sync::{Arc, LazyLock};
use tokio::time::{Duration, sleep};

/// Stores the current Spotify queue and currently playing item.
pub static CURRENT_SONGS: LazyLock<Arc<Mutex<Option<CurrentUserQueue>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

/// Stores the current playback progress in milliseconds.
pub static PLAYBACK_STATE: LazyLock<Arc<Mutex<PlaybackState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(PlaybackState::default())));

#[derive(Default)]
pub struct PlaybackState {
    playing: bool,
    shuffle: bool,
    progress: u64,
}

/// Initializes the Spotify client and spawns the combined MPRIS and Spotify polling task.
pub async fn init() -> Result<()> {
    // Initialize Spotify client with credentials and OAuth scopes
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

    // Prompt user for authorization and get the token
    let url = spotify.get_authorize_url(true).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    // Spawn the combined polling task
    tokio::spawn(polling_task(spotify));

    Ok(())
}

/// Asynchronous task to poll MPRIS every 500ms and Spotify API every 4 seconds or on song change.
async fn polling_task(spotify_client: AuthCodeSpotify) {
    let mut last_mpris_track_id: Option<String> = None; // Local state for track ID
    let mut spotify_poll_counter = 0; // Counter for regular Spotify API polling (every 4s)

    loop {
        // --- MPRIS Polling Logic ---
        let mut should_refresh_spotify = false;
        {
            let player_finder = PlayerFinder::new().expect("Could not create MPRIS player finder");
            if let Ok(players) = player_finder.find_all() {
                for player in players {
                    // Check if the current player is Spotify
                    if player.identity() == "Spotify"
                        && let Ok(metadata) = player.get_metadata()
                    {
                        // MPRIS track ID can be a URI, convert to string for comparison
                        let track_id = metadata.track_id().map(|uri| uri.to_string());

                        // Detect if the song has changed
                        if last_mpris_track_id != track_id {
                            last_mpris_track_id.clone_from(&track_id);
                            should_refresh_spotify = true;
                        }

                        // Update progress and playing status
                        if let (Ok(playback_status), Ok(position)) =
                            (player.get_playback_status(), player.get_position())
                        {
                            PLAYBACK_STATE.lock().playing =
                                matches!(playback_status, PlaybackStatus::Playing);
                            PLAYBACK_STATE.lock().progress = position.as_millis() as u64;
                        }

                        break; // Found Spotify player, no need to check other players
                    }
                }
            }
        }

        // --- Spotify API Polling Logic ---
        spotify_poll_counter += 1;
        // Trigger Spotify API refresh if counter reaches 8 (4 seconds) or a song change was detected by MPRIS
        if spotify_poll_counter >= 8 || should_refresh_spotify {
            spotify_poll_counter = 0; // Reset counter

            // Fetch the current user's queue from Spotify
            if let Ok(queue) = spotify_client.current_user_queue().await {
                *CURRENT_SONGS.lock() = Some(queue.clone());
            }
            if let Ok(Some(playback)) = spotify_client
                .current_playback(None, None::<Vec<&AdditionalType>>)
                .await
            {
                *PLAYBACK_STATE.lock() = PlaybackState {
                    playing: playback.is_playing,
                    shuffle: playback.shuffle_state,
                    progress: playback.progress.map_or(0, |p| p.num_milliseconds() as u64),
                };
            }
        }

        sleep(Duration::from_millis(500)).await;
    }
}
