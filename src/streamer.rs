extern crate hound;

use std::io;
use std::fs;
use std::i16;
use std::f32;
use std::f32::consts::PI;

pub struct Streamer {
    writer: hound::WavWriter<io::BufWriter<fs::File>>,
    sample_rate: u32,
    internal_buffer: i16,
    write_now: bool,
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
            internal_buffer: 0,
            write_now: false,
        }
    }

    pub fn stream_byte(&mut self, byte: u8) {
        if !self.write_now {
            self.internal_buffer = (byte as i16) << 8;
            self.write_now = true;
        } else {
            self.internal_buffer += byte as i16;

            // convert internal buffer to between -1 and 1
            let ibf = (self.internal_buffer as f32) / (i16::MAX as f32);

            let amplitude = i16::MAX as f32;
            let out = ((ibf as f32) * amplitude) as i16;
            //println!("output : {}", ((ibf as f32) * amplitude));
            //println!("out : {}", out);
            self.writer.write_sample(out).unwrap();

            // restore internal buffer state
            self.internal_buffer = 0;
            self.write_now = false;
        }
    }
}
