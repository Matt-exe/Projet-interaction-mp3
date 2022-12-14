use pls::{PlaylistElement, ElementLength};
use crate::musicfile::MusicFile;
use std::fs::File;
use std::io::Write;

pub fn write2playlist(music_files: Vec<MusicFile>) {

    let mut buf = Vec::new();
    let mut playlist: Vec<PlaylistElement> = Vec::new();
    
    for element in music_files {
        playlist.push(PlaylistElement {
            path: element.path().to_string(),
            title: Some(element.title()),
            len: ElementLength::Unknown,
        });
    }

    pls::write(&playlist, &mut buf).unwrap();
    let mut file = File::create("fichier.pls").unwrap();
    let _ = file.write(&buf);
}