use std::fs;
use std::fs::read_to_string;
use std::io;
use std::path::Path;
use std::time::Duration;

use crate::bencode::Bencode;
use bencode::BencodeType;
use clap::{Parser, Subcommand};
use home::home_dir;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

mod bencode;
mod sha1;

type TorrentId = u32;

/// Minimalist torrent client
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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

#[derive(Debug, Serialize, Deserialize)]
struct TorrentFileInfoFile {
    lenght: u32,
    path: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TorrentFileInfo {
    files: Vec<TorrentFileInfoFile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TorrentFile {
    id: TorrentId,
    announce: String,
    creation_date: u32,
    info: TorrentFileInfo,
    name: String,
    piece_length: u32,
    pieces: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    torrents: Vec<TorrentFile>,
}

impl AppState {
    fn load() -> Self {
        let content = read_to_string(format!(
            "{}/.local/share/torrent.rs/state.json",
            home_dir().unwrap().display()
        ))
        .unwrap();
        serde_json::from_str(&content).unwrap()
    }

    fn save(self) -> io::Result<()> {
        fs::write(
            format!(
                "{}/.local/share/torrent.rs/state.json",
                home_dir().unwrap().display()
            ),
            serde_json::to_string(&self).unwrap(),
        )
    }
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

async fn start_download() {
    sleep(Duration::from_millis(10000)).await;
}

fn add_torrent(torrent_file_path: &String) {
    let b = Bencode::from_file(Path::new(torrent_file_path)).unwrap();

    let root_dict = if let BencodeType::Dict(d) = b.node {
        d
    } else {
        panic!()
    };

    let announce = if let BencodeType::Str(str) = root_dict.get("announce").unwrap() {
        str
    } else {
        panic!()
    };

    let creation_date = if let BencodeType::Int(int) = root_dict.get("creation date").unwrap() {
        int
    } else {
        panic!()
    };

    let info = if let BencodeType::Dict(d) = root_dict.get("info").unwrap() {
        d
    } else {
        panic!()
    };

    let name = if let BencodeType::Str(d) = info.get("name").unwrap() {
        d
    } else {
        panic!()
    };

    let piece_length = if let BencodeType::Int(int) = info.get("piece length").unwrap() {
        int
    } else {
        panic!()
    };

    let pieces = if let BencodeType::Str(str) = info.get("pieces").unwrap() {
        str
    } else {
        panic!()
    };

    let mut state = AppState::load();
    state.torrents.push(TorrentFile {
        announce: announce.iter().map(|c| *c as char).collect(),
        creation_date: *creation_date as u32,
        name: name.iter().map(|c| *c as char).collect(),
        piece_length: *piece_length as u32,
        pieces: pieces.to_owned(),
        id: 123,
        info: TorrentFileInfo {
            files: vec![TorrentFileInfoFile {
                lenght: 123,
                path: vec![String::from("hello")],
            }],
        },
    });
    state.save();
}

fn rm_torrent(torrent_id: &TorrentId) {
    todo!()
}

fn ls_torrents() {
    todo!()
}

fn pause_torrent(torrent_id: &TorrentId) {
    todo!()
}

fn continue_torrent(torrent_id: &TorrentId) {
    todo!()
}

fn inspect_torrent(torrent_id: &TorrentId) {
    todo!()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    create_app_data_dir_all()?;
    create_app_state_file()?;
    let args = Args::parse();
    match &args.command {
        None => start_download().await,
        Some(Commands::Add { torrent_file_path }) => add_torrent(torrent_file_path),
        Some(Commands::Rm { torrent_id }) => rm_torrent(torrent_id),
        Some(Commands::Ls {}) => ls_torrents(),
        Some(Commands::Pause { torrent_id }) => pause_torrent(torrent_id),
        Some(Commands::Continue { torrent_id }) => continue_torrent(torrent_id),
        Some(Commands::Inspect { torrent_id }) => inspect_torrent(torrent_id),
    };
    Ok(())
    // let _a = Bencode::from_file(Path::new("fedora.torrent")).unwrap();
    // println!("Hello, world!");
}
