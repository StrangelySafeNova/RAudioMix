use std::{fs, path::PathBuf};

pub struct Config {
    files: Vec<PathBuf>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Config {
        let folder_path: String = args[1].clone();
        let mut file_vec: Vec<PathBuf> = Vec::new();

        for path in fs::read_dir(folder_path).expect("Folder not found") {
            file_vec.push(path.unwrap().path());
        }

        Config { files: file_vec }
    }

    pub fn files(&self) -> Vec<PathBuf> {
        // .clone() for the moment, for debugging purposes
        return self.files.clone();
    }
}
