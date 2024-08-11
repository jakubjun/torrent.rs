use clap::Parser;
use commands::{start_download, Args};
use std::io;

mod bencode;
mod commands;
mod sha1;
mod state;
mod torrent_file;
mod tracker_connection;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    match &args.command {
        _ => start_download().await,
        // Some(Commands::Add { torrent_file_path }) => add_torrent(torrent_file_path),
        // Some(Commands::Rm { torrent_id }) => rm_torrent(torrent_id),
        // Some(Commands::Ls {}) => ls_torrents(),
        // Some(Commands::Pause { torrent_id }) => pause_torrent(torrent_id),
        // Some(Commands::Continue { torrent_id }) => continue_torrent(torrent_id),
        // Some(Commands::Inspect { torrent_id }) => inspect_torrent(torrent_id),
    }

    // Ok(())
    // let _a = Bencode::from_file(Path::new("fedora.torrent")).unwrap();
    // println!("Hello, world!");
}
