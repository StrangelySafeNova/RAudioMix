use std::{io::BufReader, fs::File, path::PathBuf};
use hound::WavReader;

// temporary, for debug purposes
#[allow(dead_code)]
pub struct SoundFile {
    initial_sound: WavReader<BufReader<File>>,
    initial_sound_path: PathBuf,
    combined_sound: Option<WavReader<BufReader<File>>>,
    combined_sound_path: Option<PathBuf>,
}

impl SoundFile {
    pub fn new(file_path: PathBuf) -> SoundFile {
        SoundFile {
            initial_sound: WavReader::open(file_path.clone()).unwrap(),
            initial_sound_path: file_path,
            combined_sound: None,
            combined_sound_path:None,
        }
    }

    pub fn print_rms_inital(&self) {
        let mut reader = WavReader::open(self.initial_sound_path.clone()).unwrap();
        let sqr_sum = reader.samples::<i16>()
            .fold(0.0, |sqr_sum, s| {
                let sample = s.unwrap() as f64;
                sqr_sum + sample * sample
            });
        println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
    }
}
