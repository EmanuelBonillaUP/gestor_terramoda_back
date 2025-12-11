#[derive(Debug, Clone)]
pub struct SharedError {
    pub message: String,
    pub code: u16,
}
impl SharedError {
    pub fn new(message: &str, code: u16) -> Self {
        SharedError {
            message: message.to_string(),
            code,
        }
    }
}
impl std::fmt::Display for SharedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

