use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::process::{ExitCode, Termination};

#[repr(u8)]
pub enum ZapataResult {
    Success = 0,
    TickError = 1,
}

impl Termination for ZapataResult {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

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