# cantus
A beautiful interactive music widget for Wayland

<img width="1755" height="81" alt="image" src="https://github.com/user-attachments/assets/a447d690-f36c-4c72-95e3-5be8a5c9041b" />

## Features

**Graphics**: Powered by `wgpu` for high-performance, animated rendering of the music widget.

**Queue Display**: Displays your spotify queue in a visual timeline, shows upcoming songs as well as the history.

**Playback Controls**: Provides playback controls for play/pause, skip forward/backward by clicking to seek to a song, and volume adjustment with scroll. You can also smoothly drag the whole bar to seek through the timeline.

**Playlist Editing**: Favourite playlists to be displayed, shows when a song is contained in that playlist and allows you to add/remove songs from the playlist. (Also includes star ratings!)

<img width="430" height="88" alt="image" src="https://github.com/user-attachments/assets/dd8c185b-a12d-42ec-86d4-dee96ceb9ae9" />

https://github.com/user-attachments/assets/86c0bc3c-8e50-49bc-a955-86975910b7ae


## Usage

`cantus` can be run in two different modes: Wayland native (using `layer-shell` protocol) or as a standard window using `winit`.

### Getting a spotify API key

Due to spotify's rate limiting you will need to get a spotify API key from https://developer.spotify.com/dashboard/applications. And add that to the config file under the `spotify_client_id` key.

## Installing with Nix
Avaiable in nixpkgs.

As a flake for home manager:
Add to flake.nix inputs `cantus.url = "github:CodedNil/cantus";`
Enable it as a systemd module with home-manager:
```
imports = [ inputs.cantus.homeManagerModules.default ];
programs.cantus = {
  enable = true;
  package = pkgs.cantus;
  settings = {
    monitor = "eDP-1";
    width = 1050.0;
    height = 40.0;
    timeline_future_minutes = 12.0;
    timeline_past_minutes = 1.5;
    history_width = 100.0;
    playlists = [ "Rock & Roll" "Instrumental" "Pop" ];
    ratings_enabled = true;
  };
};
```

## Building from Source

To build Cantus from source, ensure the following dependencies are installed:

* Rust (with cargo)
* wayland-protocols
* clang
* libxkbcommon
* wayland
* vulkan-loader

Then, from the root of the repository, run:

```cargo build --release```

### To install it system-wide
```sudo cp target/release/cantus /usr/bin```
