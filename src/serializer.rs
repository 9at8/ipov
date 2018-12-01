#[derive(Debug)]
pub struct Serializer {
    pub input: String,
}

impl Serializer {
    pub fn serialize(&self) -> String {
        self.input.clone()
    }
}
