use std::env;
pub mod precombine;

use precombine::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(args);

    println!("{:?}", config.files());
}

