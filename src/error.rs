use std::{
    process::{ExitCode, Termination},
    cell::BorrowMutError,
    time::SystemTimeError,
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
    RuntimeError(String),
    FatalError(String),
}

impl From<SystemTimeError> for ZapataError {
    fn from(e: SystemTimeError) -> Self {
        ZapataError::RuntimeError(e.to_string())
    }
}

impl From<BorrowMutError> for ZapataError {
    fn from(e: BorrowMutError) -> Self {
        ZapataError::FatalError(e.to_string())
    }
}