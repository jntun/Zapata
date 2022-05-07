use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct MaxEntitiesError {
    details: String,
}

impl MaxEntitiesError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MaxEntitiesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MaxEntitiesError {
    fn description(&self) -> &str {
        &self.details
    }
}
