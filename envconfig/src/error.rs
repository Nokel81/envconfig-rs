use std::{error, ffi::OsString, fmt};

/// Represents an error, that may be returned by `fn init()` of trait `Envconfig`.
#[derive(Debug)]
pub enum Error {
    EnvVarMissing {
        name: OsString,
    },
    ParseError {
        name: OsString,
        info: Box<dyn error::Error>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnvVarMissing { name } => write!(f, "Env variable is missing: {:?}", name),
            Self::ParseError { name, info } => {
                write!(f, "Failed to parse env variable {:?}: {}", name, info)
            }
        }
    }
}
