use crate::shared::SharedError;

pub trait ValueObject<T> {
    fn value(&self) -> &T;
    fn validate(&self) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct Email {
    value: String,
}
impl Email {
    pub fn new(value: String) -> Result<Self, SharedError> {
        let email = Email { value };
        match email.validate() {
            Ok(_) => Ok(email),
            Err(err) => Err(SharedError::new(&err, 400)),
        }
    }
}
impl ValueObject<String> for Email {
    fn value(&self) -> &String {
        &self.value
    }

    fn validate(&self) -> Result<(), String> {
        if self.value.contains('@') {
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Phone {
    value: String,
}
impl ValueObject<String> for Phone {
    fn value(&self) -> &String {
        &self.value
    }

    fn validate(&self) -> Result<(), String> {
        if self.value.chars().all(|c| c.is_digit(10)) && self.value.len() == 10 {
            Ok(())
        } else {
            Err("Invalid phone number format".to_string())
        }
    }
}
impl Phone {
    pub fn new(value: String) -> Result<Self, SharedError> {
        let phone = Phone { value };
        match phone.validate() {
            Ok(_) => Ok(phone),
            Err(err) => Err(SharedError::new(&err, 400)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CC {
    value: String,
}
impl CC {
    pub fn new(value: String) -> Result<Self, SharedError> {
        let cc = CC { value };
        match cc.validate() {
            Ok(_) => Ok(cc),
            Err(err) => Err(SharedError::new(&err, 400)),
        }
    }
}
impl ValueObject<String> for CC {
    fn value(&self) -> &String {
        &self.value
    }
    fn validate(&self) -> Result<(), String> {
        if self.value.len() >= 5 {
            Ok(())
        } else {
            Err("CC must be at least 5 characters long".to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Url {
    value: String,
}
impl ValueObject<String> for Url {
    fn value(&self) -> &String {
        &self.value
    }

    fn validate(&self) -> Result<(), String> {
        if self.value.starts_with("http://") || self.value.starts_with("https://") {
            Ok(())
        } else {
            Err("Invalid URL format".to_string())
        }
    }
}
impl Url {
    pub fn new(value: String) -> Result<Self, SharedError> {
        let url = Url { value };
        match url.validate() {
            Ok(_) => Ok(url),
            Err(err) => Err(SharedError::new(&err, 400)),
        }
    }
}
