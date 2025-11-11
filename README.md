# cantus
A beautiful interactive music widget for Wayland

<img width="1331" height="67" alt="image" src="https://github.com/user-attachments/assets/3ccc53d0-6968-4b78-b337-08253f471fb4" />

## Features

**Graphics**: Powered by `wgpu` and `vello` for high-performance, animated rendering of the music widget.

**Queue Display**: Displays your spotify queue in a visual timeline, shows upcoming songs as well as the history.

**Playback Controls**: Provides playback controls for play/pause, skip forward/backward by clicking to seek to a song, and volume adjustment with scroll. You can also smoothly drag the whole bar to seek through the timeline.

**Playlist Editing**: Favourite playlists to be displayed, shows when a song is contained in that playlist and allows you to add/remove songs from the playlist. (Also includes star ratings!)

<img width="390" height="76" alt="image" src="https://github.com/user-attachments/assets/aa33d279-c7f2-4565-b893-f3382fc9b9ed" />

https://github.com/user-attachments/assets/bca138db-197d-403d-a6e2-c0b025df3c74

## Usage

`cantus` can be run in two different modes: Wayland native (using `layer-shell` protocol) or as a standard window using `winit`.


## Installing with Nix

As a flake:
Add to flake.nix inputs `cantus.url = "github:CodedNil/cantus";`
Enable it as a systemd module with home-manager:
```
  imports = [ inputs.cantus.homeManagerModules.default ];
  programs.cantus = {
    enable = true;
    package = inputs.cantus.packages.${pkgs.stdenv.system}.cantus;
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

    Rust (with cargo)
    wayland-protocols
    clang
    libxkbcommon
    wayland
    vulkan-loader

Then, from the root of the repository, run:

cargo build --release

# To install it system-wide
sudo cp target/release/cantus /usr/bin
