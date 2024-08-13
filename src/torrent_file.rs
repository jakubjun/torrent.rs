use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    bencode::{Bencode, BencodeType},
    state::AppState,
};
pub type TorrentId = u32;

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
pub struct TorrentFile {
    id: TorrentId,
    announce: String,
    creation_date: u32,
    info: TorrentFileInfo,
    name: String,
    piece_length: u32,
    pieces: Vec<u8>,
}

fn add_torrent(torrent_file_path: &String) {
    todo!()
    // let b = Bencode::from_file(Path::new(torrent_file_path)).unwrap();
    //
    // let root_dict = if let BencodeType::Dict(d) = b.node {
    //     d
    // } else {
    //     panic!()
    // };
    //
    // let announce = if let BencodeType::Str(str) = root_dict.get("announce").unwrap() {
    //     str
    // } else {
    //     panic!()
    // };
    //
    // let creation_date = if let BencodeType::Int(int) = root_dict.get("creation date").unwrap() {
    //     int
    // } else {
    //     panic!()
    // };
    //
    // let info = if let BencodeType::Dict(d) = root_dict.get("info").unwrap() {
    //     d
    // } else {
    //     panic!()
    // };
    //
    // let name = if let BencodeType::Str(d) = info.get("name").unwrap() {
    //     d
    // } else {
    //     panic!()
    // };
    //
    // let piece_length = if let BencodeType::Int(int) = info.get("piece length").unwrap() {
    //     int
    // } else {
    //     panic!()
    // };
    //
    // let pieces = if let BencodeType::Str(str) = info.get("pieces").unwrap() {
    //     str
    // } else {
    //     panic!()
    // };
    //
    // let mut state = AppState::load();
    // state.torrents.push(TorrentFile {
    //     announce: announce.iter().map(|c| *c as char).collect(),
    //     creation_date: *creation_date as u32,
    //     name: name.iter().map(|c| *c as char).collect(),
    //     piece_length: *piece_length as u32,
    //     pieces: pieces.to_owned(),
    //     id: 123,
    //     info: TorrentFileInfo {
    //         files: vec![TorrentFileInfoFile {
    //             lenght: 123,
    //             path: vec![String::from("hello")],
    //         }],
    //     },
    // });
    // state.save();
}
