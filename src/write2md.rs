use markdown_gen::markdown::{Markdown, AsMarkdown};
use std::{fs::File};
use crate::musicfile::{MusicFile};


pub fn write2md(musicfiles : Vec<MusicFile>) {

    let file = File::create("fichier.md").unwrap();
    let mut md = Markdown::new(file);
    for (i, music) in musicfiles.iter().enumerate() {
        md.write(format!("N° : {}, path :  {}", i, music.path()).heading(2)).unwrap();
        md.write(format!("{}{}", "Artist : ", music.artist()).paragraph()).unwrap();
        md.write(format!("{}{}", "Album : ", music.album()).paragraph()).unwrap();
        md.write(format!("{}{}", "Title : ", music.title()).paragraph()).unwrap();
        md.write(format!("{}{}", "Année : ", music.year()).paragraph()).unwrap();
    }
}