use crate::musicfile::MusicFile;
use std::fs;
use serde_json::{Value};
use std::str::FromStr;
use std::path::{Path, PathBuf};

/// Fonction pour convertir structure Value en i32
fn to_i32(value:Value) -> i32 {
    serde_json::from_value(value).unwrap()
}

/// Fonction pour convertir structure Value en String
fn to_stri(value:Value) -> String {
    serde_json::from_value(value).unwrap()
}

/// Fonction pour convertir String en PathBuf
fn to_p_buff(value:String) -> PathBuf {
    Path::new(&value).to_path_buf()
}

/// Fonction Pour recréer une structure MusicFile à partir d'une structure Value récupérer d'un fichier json
pub fn to_musique(value:Value) -> MusicFile {
    MusicFile {
        // Convertis 
        path:to_p_buff(to_stri(value["path"].clone())),
        title:to_stri(value["title"].clone()),
        year:to_i32(value["year"].clone()),
        artist:to_stri(value["artist"].clone()),
        album:to_stri(value["album"].clone())
    }
}

pub fn search(filter:Vec<String>) -> Vec<MusicFile>{

    // On récupère les données du fichier json
    let filtered = filter[0].clone();
    let data = fs::read_to_string("fichier.json").unwrap();
    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    let mut v = Vec::new();
    let split = filtered.split('=');
    let category = split.clone().collect::<Vec<&str>>()[0];
    let value = split.clone().collect::<Vec<&str>>()[1];

    for i in 0..json.as_array().unwrap().len() {
        match category {
            "year" => {
                let value_i32: i32 = FromStr::from_str(value).unwrap(); 
                if to_i32(json[i][category].clone()) == value_i32 {
                    let val = json[i].clone();
                    v.push(to_musique(val));
                }
            }
            "title" => {
                if to_stri(json[i][category].clone()) == *value {
                    let val = json[i].clone();
                    v.push(to_musique(val));
                }
            }
            "path" => {
                if to_p_buff(to_stri(json[i][category].clone())) == to_p_buff(value.to_owned()) {
                    let val = json[i].clone();
                    v.push(to_musique(val));
                } 
            }
            "artist" => {
                if to_stri(json[i][category].clone()) == *value {
                    let val = json[i].clone();
                    v.push(to_musique(val));
                }
            }
            "album" => {
                if to_stri(json[i][category].clone()) == *value {
                    let val = json[i].clone();
                    v.push(to_musique(val));
                }
            }
            _ => panic!(),
        }
    }
    v
}
