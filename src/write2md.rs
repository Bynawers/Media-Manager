use crate::musicfile::MusicFile;
use crate::write2md::markdown::Markdown;
use markdown_gen::*;
use markdown_gen::markdown::AsMarkdown;
use std::fs::File;

pub fn write_result(music_data: Vec<MusicFile>) {

    let file = File::create("result.md").unwrap();
    let mut md = Markdown::new(file);
    
    md.write("Résultat de la Requête".heading(1)).unwrap();
    
    for (i, elem) in music_data.iter().enumerate() {

        let title = uncap_string(elem.title.as_ref());
        let artist = uncap_string(elem.artist.as_ref());
        let year = uncap_number(elem.year.as_ref());
        let album = uncap_string(elem.album_artist.as_ref());

        md.write(("Musique :".italic().heading(2))
        .append(i.to_string().as_str().italic())
        .append(":\n")
        .append(title)
        .append(" by ")
        .append(artist)
        .append("\nRealise in ")
        .append(year.as_str())
        .append(" in the album ")
        .append(album)
        ).unwrap();
    }
}

fn uncap_string(entry: Option<&String>) -> &str {
    match &entry{
        None => "None",
        Some(text) => (*text).as_str(),
    }
}

fn uncap_number(entry: Option<&i32>) -> String {
    match &entry{
        None => "None".to_string(),
        Some(number) => (*number).to_string(),
    }
}
