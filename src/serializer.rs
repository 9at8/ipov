use std::io::prelude::*;
use std::fs::File;

use streamer::Streamer;

pub struct Serializer {
    pub file_name: String,
    pub streamer: Streamer,
}

impl Serializer {
    pub fn serialize(&mut self) -> std::io::Result<()> {
        let file = File::open(&self.file_name)?;

        for byte in file.bytes() {
            self.streamer.stream_byte(byte.unwrap());
        }

        Ok(())
    }
}
