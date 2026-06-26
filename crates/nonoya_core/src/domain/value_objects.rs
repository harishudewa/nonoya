pub struct ApiUrl(String);

impl ApiUrl {
    pub fn new(raw: String) -> Self {
        Self(raw)
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl From<String> for ApiUrl {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ApiUrl {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
