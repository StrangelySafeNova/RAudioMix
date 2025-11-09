use std::env;

pub mod combiner;
use combiner::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(args);

    for file in config.files() {
        file.print_rms_inital();
    }
}

