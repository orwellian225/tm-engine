use std::error::Error;
use std::fmt;

#[derive(fmt::Debug)]
pub struct TmError {
    pub message: String
}

impl TmError {
    pub fn new(message: String) -> Self { TmError { message } }
}

impl fmt::Display for TmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error has occured in the Turing Machine\n\t{}", self.message)
    }
}

impl Error for TmError {}