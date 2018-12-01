extern crate hound;

use std::io;
use std::fs;
use std::i16;
use std::f32;
use std::f32::consts::PI;

pub struct Streamer {
    writer: hound::WavWriter<io::BufWriter<fs::File>>,
    sample_rate: u32,
}

impl Streamer {

    pub fn create(file_name: String, sample_rate: u32) -> Streamer {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        Streamer {
            writer: hound::WavWriter::create(file_name, spec).unwrap(),
            sample_rate: sample_rate,
        }
    }

    // TODO: yield-style API?
    pub fn write(&mut self) -> () {
        let sample_rate = self.sample_rate;
        for t in (0 .. sample_rate).map(|x| x as f32 / (sample_rate as f32)) {
            let sample = (t * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            self.writer.write_sample((sample * amplitude) as i16).unwrap();
        }
    }

    // TODO: finalizer
}
