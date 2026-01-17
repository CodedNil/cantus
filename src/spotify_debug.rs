use crate::render::update_color_palettes;
use crate::{
    ARTIST_DATA_CACHE, Album, Artist, CondensedPlaylist, IMAGES_CACHE, PlaybackState, Track,
};
use arrayvec::ArrayString;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::thread::spawn;
use std::time::Instant;
use tracing::warn;

fn random_arraystring() -> ArrayString<22> {
    let mut s = ArrayString::<22>::new();
    for _ in 0..22 {
        let c = fastrand::alphanumeric();
        let _ = s.try_push(c);
    }
    s
}

fn artist() -> Artist {
    Artist {
        id: ArrayString::from("06HL4z0CvFAxyc27GXpf02").unwrap(),
        name: "Taylor Swift".into(),
        image: Some("https://i.scdn.co/image/ab6761610000f178e2e8e7ff002a4afda1c7147e".into()),
    }
}

fn track(name: &str, album_img: &str, duration: u32) -> Track {
    Track {
        id: random_arraystring(),
        name: name.into(),
        album: Album {
            id: random_arraystring(),
            image: Some(album_img.into()),
        },
        artist: artist(),
        duration_ms: duration,
    }
}

fn playlist(name: &str, url: &str, rating: Option<u8>) -> (ArrayString<22>, CondensedPlaylist) {
    let id = random_arraystring();
    (
        id,
        CondensedPlaylist {
            id,
            name: name.into(),
            image_url: Some(url.into()),
            tracks: HashSet::new(),
            rating_index: rating,
            tracks_total: 0,
        },
    )
}

pub fn debug_playbackstate() -> PlaybackState {
    let queue = vec![
        track(
            "King Of My Heart",
            "https://i.scdn.co/image/ab67616d00004851da5d5aeeabacacc1263c0f4b",
            214320,
        ),
        track(
            "Slut! (Taylor's Version)",
            "https://i.scdn.co/image/ab67616d00004851904445d70d04eb24d6bb79ac",
            180381,
        ),
        track(
            "Stay Beautiful",
            "https://i.scdn.co/image/ab67616d000048512f8c0fd72a80a93f8c53b96c",
            236053,
        ),
        track(
            "Speak Now",
            "https://i.scdn.co/image/ab67616d00004851be4ec62353ee75fa11f6d6f7",
            248146,
        ),
        track(
            "Superstar (Taylor’s Version)",
            "https://i.scdn.co/image/ab67616d00004851a48964b5d9a3d6968ae3e0de",
            263865,
        ),
        track(
            "Anti-Hero (feat. Bleachers)",
            "https://i.scdn.co/image/ab67616d000048519954fa9f0ba8534c3897b59d",
            228397,
        ),
        track(
            "The Man",
            "https://i.scdn.co/image/ab67616d000048519a6d517fc78fb707bed067c2",
            219385,
        ),
        track(
            "Never Grow Up",
            "https://i.scdn.co/image/ab67616d000048512e4ec3175d848eca7b76b07f",
            290466,
        ),
        track(
            "Enchanted (Taylor's Version)",
            "https://i.scdn.co/image/ab67616d000048510b04da4f224b51ff86e0a481",
            353253,
        ),
        track(
            "Only The Young",
            "https://i.scdn.co/image/ab67616d000048514aa13f6de271d8403a82e4a8",
            157507,
        ),
        track(
            "So Long, London",
            "https://i.scdn.co/image/ab67616d000048518ecc33f195df6aa257c39eaa",
            262974,
        ),
        track(
            "seven",
            "https://i.scdn.co/image/ab67616d0000485195f754318336a07e85ec59bc",
            208906,
        ),
        track(
            "marjorie",
            "https://i.scdn.co/image/ab67616d0000485125751b4b32829d6bbfe6be7f",
            257773,
        ),
        track(
            "The Archer",
            "https://i.scdn.co/image/ab67616d00004851cde19bd5377f06eb0bca3256",
            210960,
        ),
        track(
            "Holy Ground (Taylor's Version)",
            "https://i.scdn.co/image/ab67616d00004851318443aab3531a0558e79a4d",
            202960,
        ),
        track(
            "august - the long pond studio sessions",
            "https://i.scdn.co/image/ab67616d00004851045514e3ed4e1767a7c3ece5",
            260000,
        ),
        track(
            "Sparks Fly",
            "https://i.scdn.co/image/ab67616d00004851be4ec62353ee75fa11f6d6f7",
            336826,
        ),
        track(
            "ME!",
            "https://i.scdn.co/image/ab67616d00004851d8f29c3584e77996dd2f9950",
            213026,
        ),
        track(
            "Breathe",
            "https://i.scdn.co/image/ab67616d0000485134e5885465afc8a497ac1b7e",
            263986,
        ),
        track(
            "Lover (Remix) [feat. Shawn Mendes]",
            "https://i.scdn.co/image/ab67616d0000485159457bdb1edb5c6417f3baa2",
            221306,
        ),
    ];
    let mut playlists = HashMap::from_iter([
        playlist(
            "5.0",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da84c50767075b403300869b83e9",
            Some(9),
        ),
        playlist(
            "4.5",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da84f663909c16e7b5aed0bc8372",
            Some(8),
        ),
        playlist(
            "4.0",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da8419d3c6f0e8fd6c7e59950c18",
            Some(7),
        ),
        playlist(
            "3.5",
            "https://image-cdn-fa.spotifycdn.com/image/ab67706c0000da84d46b8ca2271ae9e3c1b3eeb7",
            Some(6),
        ),
        playlist(
            "3.0",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da841ad1b1f4bed4220acbf03e5d",
            Some(5),
        ),
        playlist(
            "2.5",
            "https://image-cdn-fa.spotifycdn.com/image/ab67706c0000da84a28604c3b2000226d5c91cd8",
            Some(4),
        ),
        playlist(
            "2.0",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da84d9b64f8a70e1474c4e7100cf",
            Some(3),
        ),
        playlist(
            "1.5",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da84e1f8435d59eb9f0179d6c47b",
            Some(2),
        ),
        playlist(
            "1.0",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da849994733f6f59121e779ba3f0",
            Some(1),
        ),
        playlist(
            "0.5",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da84a9438841e6e1ad17f027855d",
            Some(0),
        ),
        playlist(
            "Current",
            "https://image-cdn-ak.spotifycdn.com/image/ab67706c0000da84d382c28a67e913ea10b7b3fc",
            None,
        ),
    ]);
    // Distribute tracks into playlists
    if !playlists.is_empty() {
        let chunk_size = queue.len().div_ceil(playlists.len());
        for (i, playlist) in playlists.values_mut().enumerate() {
            let start = i * chunk_size;
            if start < queue.len() {
                let end = (start + chunk_size).min(queue.len());
                let track_ids: HashSet<ArrayString<22>> =
                    queue[start..end].iter().map(|t| t.id).collect();
                playlist.tracks_total = track_ids.len() as u32;
                playlist.tracks = track_ids;
            }
        }
    }
    // Load the images for each track, playlist, and artist
    for track in &queue {
        if let Some(image) = &track.album.image {
            ensure_image_cached(image);
        }
    }
    for playlist in playlists.values() {
        if let Some(image) = &playlist.image_url {
            ensure_image_cached(image);
        }
    }
    ARTIST_DATA_CACHE.insert(artist().id, artist().image);
    if let Some(image) = &artist().image {
        ensure_image_cached(image);
    }

    // Return the new state
    PlaybackState {
        playing: true,
        progress: 5213,
        volume: Some(100),
        queue,
        queue_index: 7,
        playlists,
        interaction: false,
        last_interaction: Instant::now(),
        last_progress_update: Instant::now(),
    }
}

fn ensure_image_cached(url: &str) {
    if IMAGES_CACHE.contains_key(url) {
        return;
    }
    IMAGES_CACHE.insert(url.to_owned(), None);

    let url = url.to_owned();
    spawn(move || {
        let agent = ureq::Agent::new_with_defaults();
        let mut response = match agent.get(&url).call() {
            Ok(response) => response,
            Err(err) => {
                warn!("Failed to cache image {url}: {err}");
                return;
            }
        };
        let Ok(dynamic_image) =
            image::load_from_memory(&response.body_mut().read_to_vec().unwrap())
        else {
            warn!("Failed to cache image {url}: failed to read image");
            return;
        };
        let dynamic_image = if dynamic_image.width() != 64 || dynamic_image.height() != 64 {
            dynamic_image.resize_to_fill(64, 64, image::imageops::FilterType::Lanczos3)
        } else {
            dynamic_image
        };
        IMAGES_CACHE.insert(url, Some(Arc::new(dynamic_image.to_rgba8())));
        update_color_palettes();
    });
}
