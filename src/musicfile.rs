use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct MusicFile {
    path: PathBuf,
}

impl MusicFile {
    pub fn new(path: &Path) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
        }
    }
}
