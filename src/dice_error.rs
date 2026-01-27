use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DiceError {
    pub message: String,
}

impl DiceError {
    pub fn new(message: &str) -> DiceError {
        DiceError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for DiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.message);
    }
}

impl Error for DiceError {}
