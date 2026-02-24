use std::path::PathBuf;
use std::{error::Error, fmt::Display, io};
type IOError = io::Error;

#[derive(Debug)]
pub enum FileSystemCause {
    RemoveFile,
    CreateFile,
    OpenFile,
    WriteFile,
    ReadFile,
    ReadDirectory,
    CreateDirectory,
    RemoveDirectory,
}
#[derive(Debug)]
pub enum ActionErrorKind {
    InvalidParams,
    MissingParams,
    InvalidGroup,
    MissingValues(PathBuf, String),
    DirEntry(IOError),
    ReadInput(IOError),
    FileSystem(FileSystemCause, PathBuf, IOError),
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

    pub fn missing_values(path: &PathBuf, msg: String) -> Self {
        ActionError {
            kind: ActionErrorKind::MissingValues(path.clone(), msg),
        }
    }

    pub fn dir_entry(io_err: IOError) -> Self {
        ActionError {
            kind: ActionErrorKind::DirEntry(io_err),
        }
    }

    pub fn read_input(io_err: IOError) -> Self {
        ActionError {
            kind: ActionErrorKind::ReadInput(io_err),
        }
    }

    fn file_system(cause: FileSystemCause, path: &PathBuf, source: IOError) -> Self {
        ActionError {
            kind: ActionErrorKind::FileSystem(cause, path.clone(), source),
        }
    }

    pub fn remove_file(path: &PathBuf, err: IOError) -> Self {
        Self::file_system(FileSystemCause::RemoveFile, path, err)
    }

    pub fn create_file(path: &PathBuf, err: IOError) -> Self {
        Self::file_system(FileSystemCause::CreateFile, path, err)
    }

    pub fn open_file(path: &PathBuf, err: IOError) -> Self {
        Self::file_system(FileSystemCause::OpenFile, path, err)
    }

    pub fn write_file(path: &PathBuf, err: IOError) -> Self {
        Self::file_system(FileSystemCause::WriteFile, path, err)
    }

    pub fn read_file(path: &PathBuf, err: IOError) -> Self {
        Self::file_system(FileSystemCause::ReadFile, path, err)
    }

    pub fn read_dir(path: &PathBuf, err: IOError) -> Self {
        Self::file_system(FileSystemCause::ReadDirectory, path, err)
    }

    pub fn create_dir(path: &PathBuf, err: IOError) -> Self {
        Self::file_system(FileSystemCause::CreateDirectory, path, err)
    }

    pub fn remove_dir(path: &PathBuf, err: IOError) -> Self {
        Self::file_system(FileSystemCause::RemoveDirectory, path, err)
    }

    pub fn kind(&self) -> &ActionErrorKind {
        &self.kind
    }
}
