# cantus
A beautiful interactive music widget for Wayland

<img width="1755" height="81" alt="image" src="https://github.com/user-attachments/assets/a447d690-f36c-4c72-95e3-5be8a5c9041b" />

## Features

**Graphics**: Heavily GPU accelerated for high-performance, animated rendering of the music widget.

**Queue Display**: Displays your spotify queue in a visual timeline, shows upcoming songs as well as the history.

**Playback Controls**: Provides playback controls for play/pause, skip forward/backward by clicking to seek to a song, and volume adjustment with scroll. You can also smoothly drag the whole bar to seek through the timeline.

**Playlist Editing**: Favourite playlists to be displayed, shows when a song is contained in that playlist and allows you to add/remove songs from the playlist. (Also includes star ratings!)

<img width="430" height="88" alt="image" src="https://github.com/user-attachments/assets/dd8c185b-a12d-42ec-86d4-dee96ceb9ae9" />

https://github.com/user-attachments/assets/86c0bc3c-8e50-49bc-a955-86975910b7ae


## Usage

`cantus` can be run in two different modes: Wayland native (using `layer-shell` protocol) or as a standard window using `winit`.

Spotify authentication opens in the browser on first launch; no developer API key is required.

## Installing with Nix
Available in nixpkgs.

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
    tempestas_enabled = true;
    timezones = [ "America/Los_Angeles" "Australia/Sydney" ];
    status_enabled = true;
    lyrics_enabled = true;
    height = 40.0;
    timeline_future_minutes = 12.0;
    timeline_past_minutes = 1.5;
    history_width = 100.0;
    playlists = [ "Rock & Roll" "Instrumental" "Pop" ];
    ratings_enabled = true;
    search_providers = [
      {
        name = "DuckDuckGo";
        url = "https://duckduckgo.com/?q={searchTerms}";
        alias = "!ddg";
        icon = "https://duckduckgo.com/favicon.ico";
      }
      {
        name = "GitHub";
        url = "https://github.com/search?q={searchTerms}";
        alias = "!gh";
        icon = "https://github.com/favicon.ico";
      }
    ];
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
* PipeWire and WirePlumber command-line tools (`pw-record` and `wpctl`)

Then, from the root of the repository, run:

```cargo build --release```

### To install it system-wide
```sudo cp target/release/cantus /usr/bin```
