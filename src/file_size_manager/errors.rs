use std::fmt::{self, Display};

pub enum RunCommandError {
    WrongEntry(String),
    ProcessFail(String),
}

impl Display for RunCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunCommandError::WrongEntry(msg) => write!(f, "wrong entry: {}", msg),
            RunCommandError::ProcessFail(msg) => write!(f, "failed to process entry: {}", msg),
        }
    }
}

pub enum ParseArgError {
    InvalidSubFunction(String),
    InvalidOption(String),
    MissingArgument(String),
}

impl Display for ParseArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseArgError::InvalidSubFunction(msg) => {
                write!(f, "invalid sub function: {}", msg)
            }
            ParseArgError::InvalidOption(msg) => write!(f, "invalid option: {}", msg),
            ParseArgError::MissingArgument(msg) => write!(f, "missing argument: {}", msg),
        }
    }
}
