use std::fs;

pub mod soundfile;
use soundfile::SoundFile;

pub struct Config {
    files: Vec<SoundFile>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Config {
        let folder_path: String = args[1].clone();
        let mut file_vec: Vec<SoundFile> = Vec::new();

        for path in fs::read_dir(folder_path).expect("Folder not found") {
            file_vec.push(SoundFile::new(path.unwrap().path()));
        }

        Config { files: file_vec }
    }

    pub fn files(self) -> Vec<SoundFile> {
        // .clone() for the moment, for debugging purposes
        return self.files;
    }
}
