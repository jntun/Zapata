use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

/*
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
} */

#[derive(Debug)]
pub struct GenericError {
    details: String,
}

impl GenericError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub type MaxEntitiesError = GenericError;
pub type TickError = GenericError;
