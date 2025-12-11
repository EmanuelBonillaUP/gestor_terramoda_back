use crate::shared::SharedError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

pub trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
}

pub trait CredentialsValidator {
    fn validate(&self, user: &String, pass: &String) -> Result<String, SharedError>;
    fn has_access(&self, api_key: &String) -> Result<(), SharedError>;
}
