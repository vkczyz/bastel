use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use rodio::source::Source;

pub struct Audio {
    pub bgm: Sink,
    bgm_stream: OutputStream,
}

impl Audio {
    pub fn new() -> Self {
        let (bgm_stream, bgm_handle) = OutputStream::try_default().unwrap();

        Audio {
            bgm: Sink::try_new(&bgm_handle).unwrap(),
            bgm_stream: bgm_stream,
        }
    }

    pub fn play_bgm(&self, path: &Path) {
        let source = match self.get_source(path) {
            Ok(s) => s.repeat_infinite(),
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        self.bgm.append(source);
        println!("Playing {}", path.display());
    }

    pub fn play_sfx(&self, path: &Path) {
        let (_stream, handle) = OutputStream::try_default().unwrap();

        let source = match self.get_source(path) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let play_result = handle.play_raw(source.convert_samples());
        if let Err(e) = play_result {
            println!("{}", e);
        }
    }

    fn get_source(&self, path: &Path) -> Result<Decoder<BufReader<File>>, Box<dyn Error>> {
        let file = BufReader::new(
            File::open(path)?);
        let source = Decoder::new(file)?;

        Ok(source)
    }
}