use std::fs;
use std::io;
use std::path::Path;

use crate::bencode::Bencode;
use clap::Parser;
use home::home_dir;
use serde::{Deserialize, Serialize};

mod bencode;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Serialize, Deserialize)]
struct AppState {
    torrent_files: Vec<String>,
}

fn create_app_data_dir_all() -> io::Result<()> {
    fs::create_dir_all(format!(
        "{}/.local/share/torrent.rs",
        home_dir().unwrap().display()
    ))?;
    Ok(())
}

fn create_app_state_file() -> io::Result<()> {
    let default = AppState {
        torrent_files: vec![],
    };
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
    create_app_data_dir_all()?;
    create_app_state_file()?;
    // let args = Args::parse();
    let _a = Bencode::from_file(Path::new("fedora.torrent")).unwrap();
    println!("Hello, world!");
    Ok(())
}
