mod deserializer;

fn main() {
    let deserializer = deserializer::Deserializer {
        input: String::from("Hello, Derializer!"),
    };
    let voice = deserializer.deserialize();

    println!("{}", voice);
}
