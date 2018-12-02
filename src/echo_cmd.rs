mod decoder;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("input file:  {}", args[1]);
    println!("output file: {}", args[2]);

    let file = File::open(args[1].to_string()).unwrap();

    let mut v: Vec<u8> = Vec::new();
    for byte in file.bytes() {
        v.push(byte.unwrap());
    }

    let mut buffer = File::create(args[2].to_string()).unwrap();
    buffer.write(v.as_slice());
}
