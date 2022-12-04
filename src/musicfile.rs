use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct MusicFile {
    pub path: PathBuf,
    pub title: String,
    pub year: i32,
    pub artist: String,
    pub album: String,
}

// les paramètres de la fonction new correspondent aux métadonnées indiquées en blanc dans le fichier scan.rs
impl MusicFile {
    pub fn new(path: &Path, title: String, year: i32, artist: String, album: String) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            title,
            year,
            artist,
            album,
        }
    }

    pub fn album(&self) -> String {
        self.album.clone()
    }

    pub fn artist(&self) -> String {
        self.artist.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn year(&self) -> String {
        self.year.to_string()
    }

    pub fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }
}
