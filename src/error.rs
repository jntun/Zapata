use std::{
    io::Error,
    fmt::Formatter,
    process::{ExitCode, Termination},
};

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
pub enum ZapataError {
    RuntimeError(Error),
    FatalError(Error),
}

impl From<Error> for ZapataError {
    fn from(e: Error) -> Self {
        ZapataError::RuntimeError(e)
    }
}