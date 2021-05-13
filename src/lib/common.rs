use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IllegalArgumentError {
    details: String,
}

impl IllegalArgumentError {
    pub fn new(details: String) -> IllegalArgumentError {
        IllegalArgumentError { details }
    }
}

impl fmt::Display for IllegalArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An illegal argument occurred: {}", self.details)
    }
}

impl Error for IllegalArgumentError {}

#[derive(Debug)]
pub struct InputError {
    details: String,
}

impl InputError {
    pub fn new(details: String) -> InputError {
        InputError { details }
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad input occurred: {}", self.details)
    }
}

impl Error for InputError {}