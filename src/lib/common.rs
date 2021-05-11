use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IllegalArgumentError {
    reason: String,
}

impl IllegalArgumentError {
    pub fn new(reason: String) -> IllegalArgumentError {
        return IllegalArgumentError { reason };
    }
}

impl fmt::Display for IllegalArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An illegal argument occurred: {}", self.reason)
    }
}

impl Error for IllegalArgumentError {}