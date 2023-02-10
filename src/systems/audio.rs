use crate::global::Global;
use crate::entity::Entity;
use crate::components::Component;
use crate::systems::System;

use std::sync::{Arc, Mutex};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;

pub struct AudioSystem {
    bgm: Sink,
    bgm_stream: OutputStream,
    global: Arc<Mutex<Global>>,
}

impl AudioSystem {
    pub fn new(global: Arc<Mutex<Global>>) -> Self {
        let (bgm_stream, bgm_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&bgm_handle).expect("Could not create audio sink"); 

        {
            let global = global.clone();
            let mut global = global.lock().expect("Could not unlock global object");
            global.signals.insert("play_bgm".to_string(), true);
        }

        AudioSystem {
            bgm: sink,
            bgm_stream: bgm_stream,
            global,
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

impl System for AudioSystem {
    fn run(&mut self, entities: &mut [Arc<Mutex<Entity>>]) {
        let mut play_bgm = false;
        let mut play_sfx = false;

        {
            let global = self.global.clone();
            let mut global = global.lock().expect("Could not unlock global object");

            if let Some(true) = global.signals.get("play_bgm") {
                play_bgm = true;
                global.signals.insert("play_bgm".to_string(), false);
            }

            if let Some(true) = global.signals.get("play_sfx") {
                play_sfx = true;
                global.signals.insert("play_sfx".to_string(), false);
            }
        }

        if !(play_bgm || play_sfx) {
            return;
        }

        for entity in entities {
            let unlocked_entity = entity.clone();
            let unlocked_entity = unlocked_entity.lock().expect("Could not acquire entity");
            for component in unlocked_entity.components.iter() {
                if let Component::Audio(audio) = component {
                    if let (true, Some(sfx)) = (play_sfx, audio.sfx.as_ref()) {
                        self.play_sfx(Path::new(sfx));
                    }

                    if let (true, Some(bgm)) = (play_bgm, audio.bgm.as_ref()) {
                        self.play_bgm(Path::new(bgm));
                    }

                    break;
                }
            }
        }
    }
}