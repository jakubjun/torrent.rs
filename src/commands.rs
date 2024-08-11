use std::{io, net::SocketAddr};

use clap::{Parser, Subcommand};
use dns_lookup::lookup_host;
use tokio::net::UdpSocket;

use crate::torrent_file::TorrentId;
/// Minimalist torrent client
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Starts to track a torrent
    Add { torrent_file_path: String },
    /// Removes a tracked torrent
    Rm { torrent_id: TorrentId },
    /// Lists tracked torrents
    Ls {},
    /// Pauses download of a tracked torrent
    Pause { torrent_id: TorrentId },
    /// Continues download of a tracked torrent
    Continue { torrent_id: TorrentId },
    /// Prints details of a tracked torrent
    Inspect { torrent_id: TorrentId },
}

pub async fn start_download() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;

    let ip = lookup_host("open.stealth.si").unwrap();
    dbg!(&ip);
    let remote_addr = ip.first().unwrap();
    sock.connect(SocketAddr::new(remote_addr.to_owned(), 80))
        .await?;
    println!("connected");
    let mut buf = [0; 1024];
    let mut payload: Vec<u8> = Vec::new();
    payload.extend((0x417 as u32).to_be_bytes());
    payload.extend((0x27101980 as u32).to_be_bytes());
    payload.extend((0 as u32).to_be_bytes());
    payload.extend((0xabab as u32).to_be_bytes());
    loop {
        let len = sock.send(&payload).await?;
        println!("{:?} bytes sent", len);

        let len = sock.recv(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, remote_addr);
    }
}

pub fn rm_torrent(torrent_id: &TorrentId) {
    todo!()
}

pub fn ls_torrents() {
    todo!()
}

pub fn pause_torrent(torrent_id: &TorrentId) {
    todo!()
}

pub fn continue_torrent(torrent_id: &TorrentId) {
    todo!()
}

pub fn inspect_torrent(torrent_id: &TorrentId) {
    todo!()
}
