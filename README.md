# Torrent.rs

<https://www.bittorrent.org/beps/bep_0003.html> for http
<https://www.bittorrent.org/beps/bep_0015.html> for udp
<https://imdl.io/book/bittorrent/udp-tracker-protocol.html> for udp packets
<https://blog.jse.li/posts/torrent/> guide

## Todos

- communicate with tracker via udp
- start download of a single file in a single thread
- download in parallel
- add http communication with tracker
- add magnet links / dht
- save state
- implement daemon to download in background
- implement visualizations (bitfield)
