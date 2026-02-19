use std::path::PathBuf;
use std::{error::Error, fmt::Display, io};
type IOError = io::Error;

#[derive(Debug)]
pub enum ActionErrorKind {
    InvalidParams,
    MissingParams,
    InvalidGroup,
    ReadInput(IOError),
    OpenFile(PathBuf, IOError),
    CreateDirectory(PathBuf, IOError),
    OverwriteDirectory(PathBuf, IOError),
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
    pub fn invalid_group() -> Self {
        ActionError {
            kind: ActionErrorKind::InvalidGroup,
        }
    }
    pub fn read_input(io_err: IOError) -> Self {
        ActionError {
            kind: ActionErrorKind::ReadInput(io_err),
        }
    }
    pub fn file_read(file_name: &PathBuf, io_err: IOError) -> Self {
        ActionError {
            kind: ActionErrorKind::OpenFile(file_name.clone(), io_err),
        }
    }
    pub fn create_dir(dir: &PathBuf, io_err: IOError) -> Self {
        ActionError {
            kind: ActionErrorKind::CreateDirectory(dir.clone(), io_err),
        }
    }
    pub fn overwrite_dir(dir_name: &PathBuf, io_err: IOError) -> Self {
        ActionError {
            kind: ActionErrorKind::OverwriteDirectory(dir_name.clone(), io_err),
        }
    }
    pub fn kind(&self) -> &ActionErrorKind {
        &self.kind
    }
}
