use std::{error::Error, fmt::Display, io};
type IOError = io::Error;

#[derive(Debug)]
pub enum ActionErrorKind {
    InvalidParams,
    MissingParams,
    OpenFile(String, IOError),
}

#[derive(Debug)]
pub struct ActionError {
    kind: ActionErrorKind,
}

impl Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ActionError: kind: {:?}", self.kind)
    }
}

impl Error for ActionError {}

impl ActionError {
    pub fn invalid_param() -> Self {
        ActionError {
            kind: ActionErrorKind::InvalidParams,
        }
    }
    pub fn missing_param() -> Self {
        ActionError {
            kind: ActionErrorKind::MissingParams,
        }
    }
    pub fn file_read(file_name: &str, io_err: IOError) -> Self {
        ActionError {
            kind: ActionErrorKind::OpenFile(file_name.to_string(), io_err),
        }
    }
    pub fn kind(&self) -> &ActionErrorKind {
        &self.kind
    }
}
