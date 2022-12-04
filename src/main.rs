use medman::{cli::CliArguments};
use medman::scan::scan;
use medman::search::search;
use std::path::Path;
use medman::write2md::write2md;
use medman::write2playlist::write2playlist;
use medman::musicfile::MusicFile;


fn main() {
    println!("Rappel :\n");
    
    println!("scan: permet de scan un dossier pour y récupérer 
    toutes les musiques en format mp3. Elles sont ensuite placer 
    sous forme de vecteur dans fichier.json.
    Exemple : scan media \n");

    println!("search: permet de filtrer fichier.json
    avec différents paramètres ( year / title / artist / album ). 
    Puis les musiques associées sont retournées. 
    Exemple : search year=2000 \n");

    println!("write2md: place les musiques filtrées dans 
    un fichier markdown (fichier.md)
    Exemple : write2md year=2000 \n");

    println!("write2playlist: place les musiques filtrées dans 
    un fichier playlist (fichier.pls)
    Exemple : write2playlist year=2000 \n");

    println!("Veuillez entrer une commande pour continuer : ");

    // scanned est une variable permettant de vérifier si les fichiers mp3 ont déjà été scanné
    let mut scanned = [false];
    let args = CliArguments::default();

    match args.command().as_str() {
        "scan" => {
            let music_files = scan(args.path());
            for music_file in &music_files {
                println!("{:?}", music_file);
            }
            scanned[0] = true;
        },
        "search" => {
            // Pour vérifier que le fichier json est déjà créer. Sinon le programme le créer.
            if !scanned[0] {
                scan(Path::new("media"));
                scanned[0] = true;
            }
            // Si en plus de la commande il n'y a pas d'arguments, appelle l'interaction utilisateur
            let request = search(args.request());
            for req in &request {
                println!("{:?}", req);
            }
        },
        "write2md" => {
            // Pour vérifier que le fichier json est déjà créer. Sinon le programme le créer.
            if !scanned[0] {
                scan(Path::new("media"));
                scanned[0] = true;
            }
            // Si en plus de la commande il n'y a pas d'arguments, appelle l'interaction utilisateur
            let result = search(args.request());
            write2md(result);
        }, 
        "write2playlist" => {
            // Pour vérifier que le fichier json est déjà créer. Sinon le programme le créer.
            if !scanned[0] {
                scan(Path::new("media"));
                scanned[0] = true;
            }
            // Si en plus de la commande il n'y a pas d'arguments, appelle l'interaction utilisateur
            let result = search(args.request());
            write2playlist(result);
        }, 
        _ => {println!("La commande saisie n'est pas valide!");} 
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn check_scan() {
        let mut music_files: Vec<MusicFile> = Vec::new();
        music_files.push(MusicFile::new(
            std::path::Path::new("media/Ambient-I.mp3"),
            "Ambient I".to_string(),
            0,
            "Steven O'Brien".to_string(),
            "None".to_string()
        ));
        
        let music = scan(std::path::Path::new("media"));
        assert_eq!(serde_json::to_string_pretty(&music_files).unwrap(), serde_json::to_string_pretty(&music).unwrap());
    }

    #[test]
    fn check_search() {
        let mut music_files = Vec::new();
        music_files.push(MusicFile::new(
            std::path::Path::new("media/Ambient-I.mp3"),
            "Ambient I".to_string(),
            0,
            "Steven O'Brien".to_string(),
            "None".to_string()
        ));

        scan(std::path::Path::new("media"));
        let music = search(vec!["title=Ambient I".to_owned()]);
        assert_eq!(serde_json::to_string_pretty(&music_files).unwrap(), serde_json::to_string_pretty(&music).unwrap());
    }

    #[test]
    fn check_write2md() {
        if std::fs::read("fichier.md").is_ok() {
            let _ = std::fs::remove_file("fichier.md");
        }
        scan(std::path::Path::new("media"));
        let music = search(vec!["title=Ambient I".to_owned()]);
        write2md(music);

        assert!(std::fs::read("fichier.md").is_ok());
    }

    #[test]
    fn check_write2playlist() {
        if std::fs::read("fichier.pls").is_ok() {
            let _ = std::fs::remove_file("fichier.pls");
        }
        scan(std::path::Path::new("media"));
        let music = search(vec!["title=Ambient I".to_owned()]);
        write2playlist(music);

        assert!(std::fs::read("fichier.pls").is_ok());
    }

}
