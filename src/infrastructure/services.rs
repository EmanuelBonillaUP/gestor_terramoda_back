use crate::application::services::{CredentialsValidator, LogLevel, Logger};

pub struct LoggerConfig {
    pub format: String,
    pub min_level: LogLevel,
}

pub struct LoggerPrinter {
    config: LoggerConfig,
}
impl LoggerPrinter {
    pub fn new(config: LoggerConfig) -> Self {
        LoggerPrinter { config }
    }
}
impl Logger for LoggerPrinter {
    fn log(&self, level: LogLevel, message: &str) {
        let format = self
            .config
            .format
            .replace("{level}", &format!("{:?}", level))
            .replace("{timestamp}", &chrono::Utc::now().to_rfc3339())
            .replace("{message}", message);
        println!("{}", format);
    }
}

pub struct CredentialsValidatorOneUserConfig {
    pub user: String,
    pub pass: String,
    pub key: String,
}

pub struct CredentialsValidatorOneUser {
    pub config: CredentialsValidatorOneUserConfig,
}
impl CredentialsValidator for CredentialsValidatorOneUser {
    fn validate(&self, user: &String, pass: &String) -> Result<String, crate::shared::SharedError> {
        if &self.config.user == user && &self.config.pass == pass {
            Ok(self.config.key.clone())
        } else {
            Err(crate::shared::SharedError::new("INVALID CREDENTIALS", 401))
        }
    }
    fn has_access(&self, api_key: &String) -> Result<(), crate::shared::SharedError> {
        if &self.config.key != api_key {
            Err(crate::shared::SharedError::new("NOT AUTHORIZED", 401))
        } else {
            Ok(())
        }
    }
}
