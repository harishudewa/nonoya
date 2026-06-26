#[derive(Clone, Debug)]
pub struct ContentId(String);

impl ContentId {
    pub fn new(raw: String) -> Self {
        Self(raw)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl From<String> for ContentId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ContentId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
