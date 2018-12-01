mod serializer;

fn main() {
    let serializer = serializer::Serializer {
        input: String::from("Hello, Serializer!"),
    };
    let voice = serializer.serialize();

    println!("{}", voice);
}
