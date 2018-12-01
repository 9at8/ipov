extern crate ears;

use std::thread::sleep;
use std::time::Duration;
use ears::AudioController;

fn main() {
    // initialize the RecordContext
    let ctxt = ears::init_in().expect("Initialization error!");

    // Create a new Recorder using the RecordContext
    let mut recorder = ears::Recorder::new(ctxt);
    println!("Recording for 3 seconds");
    recorder.start();
    sleep(Duration::from_millis(3000));
    recorder.stop();
    match recorder.save_to_file("hello") {
        true => println!("Save okay!"),
        false => println!("Cannot save ...")
    }

    println!("Playing hello.wav");
    let mut sound = ears::Sound::new("hello.wav").unwrap();
    sound.play();
    while sound.is_playing() {}
}