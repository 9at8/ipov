extern crate iron;

use streamer::Streamer;
use server::Server;

mod serializer;
mod streamer;
mod server;

fn main() {
    let encoded_file_name = String::from("./encoded.wav");
    let file_to_encode = String::from("./to_encode.jpg");

    let streamer = Streamer::create(encoded_file_name, 16000);
    let mut serializer = serializer::Serializer {
        file_name: file_to_encode,
        streamer: streamer,
    };

    serializer.serialize();

    let server = Server {};
    server.start();
 }
