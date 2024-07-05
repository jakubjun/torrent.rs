use std::path::Path;

use crate::bencode::Bencode;

mod bencode;

fn main() {
    let _a = Bencode::from_file(Path::new("without_pieces.torrent")).unwrap();
    println!("Hello, world!");
}
