use std::fs;
use std::io;
use std::path::Path;

use crate::bencode::Bencode;
use clap::{Parser, Subcommand};
use home::home_dir;
use serde::{Deserialize, Serialize};

mod bencode;

/// Minimalist torrent client
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts to track a torrent
    Add { torrent_file_path: String },
    /// Removes a tracked torrent
    Rm { torrent_id: u32 },
    /// Lists tracked torrents
    Ls {},
    /// Pauses download of a tracked torrent
    Pause { torrent_id: u32 },
    /// Continues download of a tracked torrent
    Continue { torrent_id: u32 },
    /// Prints details of a tracked torrent
    Inspect { torrent_id: u32 },
    /// Prints contents of config file
    Config {},
    /// Prints contents of state file
    State {},
}

#[derive(Serialize, Deserialize)]
struct TorrentFileInfoFile {
    lenght: u32,
    path: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct TorrentFileInfo {
    files: Vec<TorrentFileInfoFile>,
}

#[derive(Serialize, Deserialize)]
struct TorrentFile {
    id: u32,
    announce: String,
    creation_date: u32,
    info: TorrentFileInfo,
    name: String,
    piece_length: u32,
    pieces: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct AppState {
    torrents: Vec<String>,
}

fn create_app_data_dir_all() -> io::Result<()> {
    fs::create_dir_all(format!(
        "{}/.local/share/torrent.rs",
        home_dir().unwrap().display()
    ))?;
    Ok(())
}

fn create_app_state_file() -> io::Result<()> {
    let default = AppState { torrents: vec![] };
    if !Path::new(&format!(
        "{}/.local/share/torrent.rs/state.json",
        home_dir().unwrap().display()
    ))
    .exists()
    {
        fs::write(
            format!(
                "{}/.local/share/torrent.rs/state.json",
                home_dir().unwrap().display()
            ),
            serde_json::to_string(&default).unwrap(),
        )
        .expect("Unable to write file");
    };
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    create_app_data_dir_all()?;
    create_app_state_file()?;
    let _a = Bencode::from_file(Path::new("fedora.torrent")).unwrap();
    println!("Hello, world!");
    Ok(())
}
