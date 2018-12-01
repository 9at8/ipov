mod serializer;
mod streamer;

fn main() {
    use streamer::*;

    let serializer = serializer::Serializer {
        input: String::from("Hello, Serializer!"),
    };
    let voice = serializer.serialize();

    let mut st = Streamer::create(
        "sine.wav".to_string(),
        512);
    st.write();

    println!("{}", voice);
}
