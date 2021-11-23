use crate::musicfile::MusicFile;

pub fn search_args(line: &[String], music_data: &[MusicFile]) -> Vec<MusicFile> {

    let mut request_data: Vec<MusicFile> = Vec::new();
    let mut is_command: bool = true;
    let mut index: usize = 0;

    for command_value in line.iter() {

        match is_command {

            true => {
                if command_value.to_uppercase().as_str() == "ALL" {
                    data_all(music_data, &mut request_data);
                    break;
                }
                else if line.contains(&"AND".to_string()) {
                    struct_request(&mut request_data, music_data, &line);
                    break;
                }
                is_command = false;
            },

            false => {
                data_music(&line[index], music_data, &mut request_data, line[index-1].to_uppercase().as_str());
                is_command = true;
            }
        }
        index += 1;
    }
    request_data
}

fn data_all(music_data: &[MusicFile], request_data: &mut Vec<MusicFile>) {

    for elem in music_data.iter() {
        request_data.push(MusicFile::new(&elem.path));
    }
}

fn data_music(request: &String, music_data: &[MusicFile], request_data: &mut Vec<MusicFile>, cmd: &str) {
    
    for elem in music_data.iter() {
        match cmd {
            "TITLE" => { match &elem.title {
                None => (),
                Some(title) => { if title.to_lowercase() == request.to_lowercase() {
                    request_data.push(MusicFile::new(&elem.path))}
                    },
                }
            },
            "ARTIST" => { match &elem.artist {
                None => (),
                Some(artist) => { if artist.to_lowercase() == request.to_lowercase() {
                    request_data.push(MusicFile::new(&elem.path))}
                    },
                }
            },
            "ALBUM" => { match &elem.album_artist {
                None => (),
                Some(album) => { if album.to_lowercase() == request.to_lowercase() {
                    request_data.push(MusicFile::new(&elem.path))}
                    },
                }
            }
            "YEAR" => { match &elem.year {
                None => (),
                Some(year) => { if *year == request.parse::<i32>().unwrap() {
                    request_data.push(MusicFile::new(&elem.path))}
                    },
                }
            },
            _ => (),
        }
    }
}
fn struct_request(request_data: &mut Vec<MusicFile>,music_data: &[MusicFile], line: &[String]) {

    let mut request: (Option<String>, Option<String>, 
        Option<String>, Option<i32>) = (None, None, None, None);

    for (i, command_value) in line.iter().enumerate() {

        match command_value.to_uppercase().as_str() {

            "TITLE" => match request.0{
                None => request.0 = Some(line[i+1].to_string().clone()),
                Some(_) => ()
            }
            "ARTIST" => match request.1{
                None => request.1 = Some(line[i+1].to_string()),
                Some(_) => ()
            }
            "ALBUM" => match request.1{
                None => request.2 = Some(line[i+1].to_string()),
                Some(_) => ()
            }
            "YEAR" => match request.1{
                None => request.3 = Some((line[i+1]).parse::<i32>().unwrap()),
                Some(_) => ()
            }
            "AND" => {()}
            _ => ()
        }
    }
    struct_to_index(request_data, music_data, request);
}

fn struct_to_index(request_data: &mut Vec<MusicFile>, music_data: &[MusicFile], request: (Option<String>, Option<String>, Option<String>, Option<i32>)) {

    let mut index_list = vec![true; music_data.len()];

    for (i, elem) in music_data.iter().enumerate() {

        if request.0 != None { match &elem.title {
            None => {index_list[i] = false},
            Some(title) => { if title.to_lowercase() != request.0.as_ref().unwrap().to_lowercase(){
                index_list[i] = false;}
                },
            }
        }
        if request.1 != None { match &elem.artist {
            None => {index_list[i] = false},
            Some(artist) => { if artist.to_lowercase() != request.1.as_ref().unwrap().to_lowercase(){
                index_list[i] = false}
                },
            }
        }
        if request.2 != None { match &elem.album_artist {
            None => {index_list[i] = false},
            Some(album) => { if album.to_lowercase() != request.2.as_ref().unwrap().to_lowercase(){
                index_list[i] = false}
                },
            }
        }
        if request.3 != None { match &elem.year {
            None => {index_list[i] = false},
            Some(year) => { if year != request.3.as_ref().unwrap(){
                index_list[i] = false}
                },
            }
        }
    }
    index_to_data(request_data, music_data, index_list);
}

fn index_to_data(request_data: &mut Vec<MusicFile>, music_data: &[MusicFile], index_list: Vec<bool>){

    for (i, elem) in index_list.iter().enumerate() {
        if *elem == true {
            request_data.push(MusicFile::new(&music_data[i as usize].path));
        }
    }
}