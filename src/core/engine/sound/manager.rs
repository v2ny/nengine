use std::{fs::File, path::Path};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use crate::{log, utils::log::manager::{LogLevel, Logger}};

#[derive(Debug, Clone)]
pub struct SoundManager {
    pub id: u32,
    pub sounds: Vec<Sound>,
    logger: Logger,
}

#[derive(Debug, Clone)]
pub struct Sound {
    pub id: u32,
    pub source: String,
    pub volume: f32,
    pub muted: bool,
    pub play_on_start: bool,
    logger: Logger,
}


impl Default for SoundManager {
	fn default() -> Self {
		Self {
            id: 0,
            sounds: Vec::new(),
            logger: Logger::new("debug/sound_manager.log"),
        }
	}
}

impl SoundManager {
    pub fn add(&mut self, mut sound: Sound) {
        if sound.play_on_start {
            sound.play();
        }
        self.sounds.push(sound.clone());
        log!(
            self.logger.clone(),
            LogLevel::Info,
            "[SM:ADD] Added \"{}\" sound to the sounds vector.",
            sound.source
        );
    }

    pub fn remove(&mut self, sound_id: u32) {
        self.sounds.retain(|sound| sound.id != sound_id);
        log!(
            self.logger.clone(),
            LogLevel::Info,
            "[SM:REMOVE] Removed sound with ID: {}.",
            sound_id
        );
    }
}

impl Sound {
    pub fn new(id: u32, source: String, volume: f32, play_on_start: bool) -> Self {
        Sound {
            id,
            source: source.clone(),
            volume,
            play_on_start,
            muted: false,
            logger: Logger::new(format!("debug/sound_{}.log", Path::new(&source).file_name().expect("Failed to find any file names in sound path").to_str().unwrap()).as_str()),
        }
    }

    pub fn play(&mut self) {
        if self.muted {
            log!(
                self.logger.clone(),
                LogLevel::Info,
                "[MUTED AUDIO] \"{}\" won't be played till unmuted.",
                self.source.clone()
            );
            return;
        }

        let file = File::open(self.source.clone()).unwrap_or_else(|err| {
            log!(
                self.logger.clone(),
                LogLevel::Error,
                "[ERROR] Failed to open sound file \"{}\": {}",
                self.source,
                err
            );
            panic!("[ERROR] Sound file could not be opened.")
        });

        let vclone = self.volume;
        let sclone = self.source.clone();
		let mut slogger = self.logger.clone();

        std::thread::spawn(move || {
            // Create an output stream
            let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|err| {
                log!(
                    slogger,
                    LogLevel::Error,
                    "[ERROR] Failed to create audio stream: {}",
                    err
                );
                panic!("[ERROR] Audio stream could not be created.")
            });

            // Open the sound file
            let source = Decoder::new(BufReader::new(file)).unwrap_or_else(|err| {
                log!(
                    slogger,
                    LogLevel::Error,
                    "[ERROR] Failed to decode audio file \"{}\": {}",
                    sclone,
                    err
                );
                panic!("[ERROR] Audio file decoding failed.")
            });

            // Create a sink for controlling volume
            let sink = Sink::try_new(&stream_handle).unwrap_or_else(|err| {
                log!(
                    slogger,
                    LogLevel::Error,
                    "[ERROR] Failed to create sink for audio playback: {}",
                    err
                );
                panic!("[ERROR] Sink creation failed.")
            });
            sink.set_volume(vclone);

            log!(
                slogger,
                LogLevel::Info,
                "[PLAYING] \"{}\" at volume \"{}\".",
                sclone,
                vclone
            );

            // Play the sound
            sink.append(source);
            sink.sleep_until_end();
        });
    }

    pub fn mute(&mut self) {
        self.muted = true;
        log!(
            self.logger.clone(),
            LogLevel::Info,
            "[MUTE] Muted sound \"{}\".",
            self.source
        );
    }

    pub fn unmute(&mut self) {
        self.muted = false;
        log!(
            self.logger.clone(),
            LogLevel::Info,
            "[UNMUTE] Unmuted sound \"{}\".",
            self.source
        );
    }
}
