use std::{
    fs::{self, read_to_string},
    io,
    path::Path,
};

use crate::torrent_file::TorrentFile;
use home::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub torrents: Vec<TorrentFile>,
}

impl AppState {
    pub fn create_if_not_exists() -> io::Result<()> {
        create_app_data_dir_all()?;
        create_app_state_file()?;
        Ok(())
    }

    pub fn load() -> Self {
        let content = read_to_string(format!(
            "{}/.local/share/torrent.rs/state.json",
            home_dir().unwrap().display()
        ))
        .unwrap();
        serde_json::from_str(&content).unwrap()
    }

    pub fn save(self) -> io::Result<()> {
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
