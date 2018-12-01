mod serializer;
mod streamer;

fn main() {
    use streamer::*;

    let streamer = Streamer::create(String::from("sine.wav"), 10000);
    let mut serializer = serializer::Serializer {
        file_name: String::from("/Users/archerzhang/Downloads/pic.jpg"),
        streamer: streamer,
    };

    serializer.serialize();

    // println!("{}", voice);
}
