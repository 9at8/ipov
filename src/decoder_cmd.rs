mod decoder;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("input file:  {}", args[1]);
    println!("output file: {}", args[2]);

    let mut dd = decoder::Decoder::new(args[1].to_string(), args[2].to_string());
    dd.decode();
}
