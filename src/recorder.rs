extern crate ears;

use std::thread::sleep;
use std::time::Duration;

pub struct Recorder {
    file_name: String,
    recorder: ears::Recorder,
    recording_length: u64,
}

impl Recorder {
    pub fn new(file_name: String, recording_length: u64) -> Recorder {
        let ctx = ears::init_in().expect("Initialization error!");
        Recorder {
            recorder: ears::Recorder::new(ctx),
            file_name: file_name,
            recording_length: recording_length,
        }
    }

    pub fn listen_and_save(&mut self) -> () {
        self.recorder.start();
        sleep(Duration::from_millis(self.recording_length));
        self.recorder.stop();
        match self.recorder.save_to_file(self.file_name.as_str()) {
            true => println!("Save okay!"),
            false => println!("Cannot save ...")
        }
    }
}
