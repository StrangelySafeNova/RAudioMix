use std::{fs, env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(args);

    println!("{:?}", config.files);
}

struct Config {
    files: Vec<PathBuf>,
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        let folder_path: String = args[1].clone();
        let mut file_vec: Vec<PathBuf> = Vec::new();
        for path in fs::read_dir(folder_path).expect("Folder not found") {
            file_vec.push(path.unwrap().path());
        }

        Config { files: file_vec }
    }
}
