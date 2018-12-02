mod serializer;
mod streamer;
mod decoder;

use streamer::Streamer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("input file:  {}", args[1]);
    println!("output file: {}", args[2]);

    let streamer = Streamer::create(String::from(args[2].to_string()), 10000);
    let mut serializer = serializer::Serializer {
        file_name: args[1].to_string(),
        streamer: streamer,
    };
    serializer.serialize();
}
