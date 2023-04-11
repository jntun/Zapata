use std::{
    cell::BorrowMutError,
    fmt::{Display, Formatter, Write},
    process::{ExitCode, Termination},
    time::SystemTimeError,
};

#[repr(u8)]
pub enum ZapataResult {
    Success = 0,
    Fatal = 1,
    Runtime = 2,
}

impl From<ZapataError> for ZapataResult {
    fn from(e: ZapataError) -> Self {
        match e {
            ZapataError::FatalError(e) => ZapataResult::Fatal,
            ZapataError::RuntimeError(e) => ZapataResult::Success,
        }
    }
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

impl Display for ZapataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeError(msg) => f.write_str(format!("Runtime error: {}", msg).as_str()),
            Self::FatalError(msg) => f.write_str(format!("Fatal error: {}", msg).as_str()),
        }
    }
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
