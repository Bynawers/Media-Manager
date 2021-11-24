use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::fs;
use audiotags::*;

#[derive(Serialize,Deserialize, Debug)]
pub struct MusicFile {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub album_artist: Option<String>,
}

/*Sauvegarde de variables dans un fichier json*/
pub fn save(music_file: Vec<MusicFile>) -> serde_json::value::Value{
    let serialized = music_file.serialize(serde_json::value::Serializer).unwrap();
    
    let serialized_pretty = serde_json::to_string_pretty(&serialized);
    match serialized_pretty {
        Err(_) => println!("error"),
        Ok(data) => {
            let data_string = data;
            fs::write("./data.json",data_string).expect("no file find for saving")},
    }
    serialized
}

/*Chargement de variables à partir d'un fichier json*/
pub fn load() -> Option<Vec<MusicFile>> {
    let file_value_res = fs::read_to_string("./data.json");
    match file_value_res {
        Err(_) => None,
        Ok(file_value) => {
            let deserialized: Vec<MusicFile> = serde_json::from_str(&file_value).unwrap();
            Some(deserialized)
        }
    }
}
impl MusicFile {

    pub fn new(path: &Path) -> MusicFile {

        let tag = Tag::default().read_from_path(&path);

        match tag {
            Err(_) => return MusicFile{ 
                path: path.to_path_buf(), 
                title: None, 
                artist: None,
                year: None,
                album_artist: None,
            },
            Ok(data) => MusicFile {
                path: path.to_path_buf(),
                title: data.title().map(|name| name.to_string()),
                artist: data.artist().map(|name| name.to_string()),
                year: data.year(),
                album_artist: data.album_artist().map(|name| name.to_string()),
            }
        }
    }
}
