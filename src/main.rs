use medman::cli::CliArguments;
use medman::scan::scan;
use medman::musicfile::*;
use colored::*;

fn main() {
    let args = CliArguments::new();
    println!("\n{} {:?}\n", "ARGUMENTS".yellow(), args);

    match args {

        CliArguments::Scan {ref path} => {
            println!("{}", "SCAN".yellow());
            let music_files = scan(path);

            for music_file in &music_files {
                println!("{:?}\n", music_file);
            }
            save(music_files);
        },

        CliArguments::Search {line} => {
            let music_data: Option<Vec<MusicFile>> = load();
            match &music_data{
                None => (),
                Some(_data_file) =>{
                    let request_data = medman::search::search_args(&line, &music_data.as_ref().expect("no music data"));
                    medman::write2md::write_result(request_data);
                }
            }
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_music() {
        
        let music_data: Option<Vec<MusicFile>> = load();

        let mut line: Vec<String> = Vec::new();
        line.push("title".to_string());
        line.push("Invincible".to_string());

        let request = medman::search::search_args(&line, &music_data.expect("no music data"));

        assert_eq!(request[0].title, Some("Invincible".to_string()));
    }

    #[test]
    fn multiple_search_music() {
        
        let music_data: Option<Vec<MusicFile>> = load();

        let mut line: Vec<String> = Vec::new();
        line.push("title".to_string());
        line.push("Invincible".to_string());
        line.push("artist".to_string());
        line.push("Hans Zimmer".to_string());

        let request = medman::search::search_args(&line, &music_data.expect("no music data"));

        assert_eq!(request[0].title, Some("Invincible".to_string()));
        assert_eq!(request[1].artist, Some("Hans Zimmer".to_string()));
    }

    #[test]
    fn search_music_with_AND() {
        
        let music_data: Option<Vec<MusicFile>> = load();

        let mut line: Vec<String> = Vec::new();
        line.push("title".to_string());
        line.push("Armada".to_string());
        line.push("AND".to_string());
        line.push("artist".to_string());
        line.push("Hans Zimmer".to_string());

        let request = medman::search::search_args(&line, &music_data.expect("no music data"));

        assert_eq!(request[0].title, Some("Armada".to_string()));
        assert_eq!(request[0].artist, Some("Hans Zimmer".to_string()));
        assert_eq!(request.len(), 1);
    }

    #[test]
    fn empty() {
        
        let music_data: Option<Vec<MusicFile>> = load();

        let mut line: Vec<String> = Vec::new();

        let request = medman::search::search_args(&line, &music_data.expect("no music data"));

        assert_eq!(request.len(), 0);
    }
}