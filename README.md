# cantus
A beautiful interactive music widget for Wayland

<img width="1331" height="67" alt="image" src="https://github.com/user-attachments/assets/3ccc53d0-6968-4b78-b337-08253f471fb4" />

## Features

**Graphics**: Powered by `wgpu` and `vello` for high-performance, beautiful rendering of the music widget.

**Queue Display**: Displays your spotify queue in a beautiful timeline, shows upcoming songs as well as the history.

**Playback Controls**: Provides playback controls for play/pause, skip forward/backward by clicking to seek to a song, and volume adjustment with scroll. You can also smoothly drag the whole bar to seek through the timeline.

**Playlist Editing**: Favourite playlists to be displayed, shows when a song is contained in that playlist and allows you to add/remove songs from the playlist. (Also includes star ratings!)

<img width="390" height="76" alt="image" src="https://github.com/user-attachments/assets/aa33d279-c7f2-4565-b893-f3382fc9b9ed" />

https://github.com/user-attachments/assets/bca138db-197d-403d-a6e2-c0b025df3c74

## Usage

`cantus` can be run in two different modes: Wayland native (using `layer-shell` protocol) or as a standard window using `winit`.
