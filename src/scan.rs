use std::{path::Path, fs::File, io::Write};
use walkdir::{DirEntry, WalkDir};
use id3::{Tag, TagLike};
use crate::musicfile::MusicFile;

const SUPPORTED_EXTENSIONS: [&str; 1] = ["mp3"];

fn is_supported(entry: &DirEntry) -> bool {
    entry.path().is_file() &&
    SUPPORTED_EXTENSIONS.contains(&entry.path().extension().unwrap().to_str().unwrap())
}

/// Fonction qui prend en paramètre un chemin et qui retourne un Vecteur composé d'une structure MusicFile (contenant métadonnées).
/// id3:Tag et TagLike pour récupérer les métadonnées et crée une structure MusicFile qui sera ajoutée dans le vecteur musicfile.
/// Une fois tous les fichiers traités, on sauvegarde les données du vecteur dans un fichier JSON.
pub fn scan(path: &Path) -> Vec<MusicFile> {
    let mut music_files: Vec<MusicFile> = Vec::new();
    let walker = WalkDir::new(path).into_iter();
    for entry in walker {
        let file = match entry {
            Ok(file) => {file},
            Err(err) => {println!("{}, le fichier n'est pas conforme.", err);
                                break },
        }; 
        if is_supported(&file) {
            // Liste des métadonnées récupérés avec la biblothèque Tag, pour chaque fichier du dossiers, en utilisant le chemin du fichier.
            // Renvoi le nom du fichier si métadonnées inutilisables.
            let value = match Tag::read_from_path(file.path()) {
                Ok(elt) => {elt},
                Err(_pb) => {break;},
            };
            // Rajoute les métadonnées voulues dans le vecteur music_files 
            music_files.push(MusicFile::new(
                    file.path(),
                    match value.title() {
                        Some(e) => e.to_string(), 
                        None => "None".to_string(),
                    },
                    value.year().unwrap_or(0),
                    match value.artist() { 
                        Some(e) => e.to_string(), 
                        None => "None".to_string(),
                    },                        
                    match value.album() { 
                        Some(e) => e.to_string(), 
                        None => "None".to_string(),
                    },
                    ));
        }
    };
    let serialized = serde_json::to_string_pretty(&music_files).unwrap();
    let mut file = File::create("fichier.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();

    music_files
}
