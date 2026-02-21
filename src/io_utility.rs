use crate::errors::ActionError;
use crate::paths;
use std::path::PathBuf;
use std::result;

type Result<T> = result::Result<T, ActionError>;

use std::{
    fs,
    io::{self},
    path::Path,
};

pub fn is_save_present() -> bool {
    Path::new(&paths::get_save_path()).exists()
}

pub fn create_save() -> Result<()> {
    let save_path = &paths::get_save_path();
    fs::create_dir(save_path).map_err(|e| ActionError::create_dir(save_path, e))
}

pub fn request_input() -> Result<String> {
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .map_err(|e| ActionError::read_input(e));
    Ok(response)
}

pub fn overwrite_dir(path: &PathBuf) -> Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).map_err(|e| ActionError::remove_dir(path, e))?;
    }
    fs::create_dir(path).map_err(|e| ActionError::create_dir(path, e))
}

pub fn get_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut vec = Vec::new();
    for entry in fs::read_dir(path).map_err(|e| ActionError::read_dir(path, e))? {
        let entry = entry.map_err(|e| ActionError::dir_entry(e))?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            vec.push(entry_path);
        }
    }
    Ok(vec)
}
