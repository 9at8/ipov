#[derive(Debug)]
pub struct Deserializer {
    pub input: String,
}

impl Deserializer {
    pub fn deserialize(&self) -> String {
        self.input.clone()
    }
}
