extern crate hound;

use std::io;
use std::fs;
use std::i16;
use std::f32;
use std::f32::consts::PI;

pub struct Streamer {
    writer: hound::WavWriter<io::BufWriter<fs::File>>,
}

impl Streamer {

    pub fn create(file_name: String) -> Streamer {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        Streamer {
            writer: hound::WavWriter::create(file_name, spec).unwrap(),
        }
    }

    // TODO: yeild-style API?
    pub fn write(&mut self) -> () {
        for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
            let sample = (t * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            self.writer.write_sample((sample * amplitude) as i16).unwrap();
        }
    }

    // TODO: finalizer

}
