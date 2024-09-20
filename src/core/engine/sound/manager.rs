use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

#[derive(Clone, Debug, Default)]
pub struct SoundManager {
    pub id: u32,
    pub sounds: Vec<Sound>,
}

#[derive(Clone, Debug, Default)]
pub struct Sound {
    pub id: u32,
    pub source: String,
    pub volume: f32,
    pub muted: bool,
    pub play_on_start: bool,
}

impl SoundManager {
    pub fn new() -> Self {
        SoundManager {
            ..Default::default()
        }
    }

    pub fn add(&mut self, sound: Sound) {
        if sound.play_on_start {
            let mut sound_to_play = sound.clone();
            sound_to_play.play();
        }
        self.sounds.push(sound.clone());
		println!("[SM:ADD] Added \"{}\" sound to the sounds vector.", sound.source);
    }

    pub fn remove(&mut self, sound_id: u32) {
        self.sounds.retain(|sound| sound.id != sound_id);
    }
}

impl Sound {
    pub fn new(sound: Sound) -> Self {
        sound
    }

    pub fn play(&mut self) {
        if self.muted {
            println!("[MUTED AUDIO] \"{}\" won't be played till unmuted.", self.source.clone());
            return;
        }

		let file = File::open(self.source.clone()).unwrap();
		let vclone = self.volume;
		let sclone = self.source.clone();
        std::thread::spawn(move || {
			// Create an output stream
			let (_stream, stream_handle) = OutputStream::try_default().unwrap();

			// Open the sound file
			let source = Decoder::new(BufReader::new(file)).unwrap();

			// Create a sink for controlling volume
			let sink = Sink::try_new(&stream_handle).unwrap();
			sink.set_volume(vclone);

			println!("[PLAYING] \"{}\" at volume \"{}\".", sclone, vclone.clone());

			// Play the sound
			sink.append(source);
			sink.sleep_until_end();
		});
    }
}