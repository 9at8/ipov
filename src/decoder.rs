extern crate hound;

use std::io;
use std::i16;
use std::u8;
use std::fs::File;
use std::io::prelude::*;

pub struct Decoder {
    reader: hound::WavReader<io::BufReader<File>>,
    output: String,
}

impl Decoder {
    pub fn new(file_path: String, output: String) -> Decoder {
        Decoder {
            reader: hound::WavReader::open(file_path).unwrap(),
            output: output,
        }
    }

    pub fn decode(&mut self) {
        let samples = self.reader.samples::<i16>();
        let mut res: Vec<u8> = Vec::new();
        for sample in samples {
            let amplitude = i16::MAX as f32;

            let sample_unwrapped = sample.unwrap();

            let val = ((sample_unwrapped as f32 / amplitude) * (i16::MAX as f32)) as i16;


            let upper_val: u8 = ((val & 0xff00) >> 8) as u8;
            let lower_val: u8 = (val & 0xff) as u8;

            res.push(upper_val);
            res.push(lower_val);
        };
        let mut buffer = File::create(self.output.clone()).unwrap();
        println!("len: {}", res.len());
        buffer.write(res.as_slice());
    }


}
