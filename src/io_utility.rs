use crate::paths;
use std::os::unix::fs::FileExt;
use std::path::PathBuf;

use std::{
    fs,
    io::{self, Error},
    path::Path,
};

pub fn is_save_present() -> bool {
    Path::new(&paths::get_save_path()).exists()
}

pub fn create_save() -> Result<(), Error> {
    fs::create_dir(&paths::get_save_path())
}

pub fn request_input() -> Result<String, Error> {
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    Ok(response)
}

pub fn overwrite_dir<P: AsRef<Path> + ?Sized>(path: &P) -> Result<(), Error> {
    if path.as_ref().exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir(path)
}

pub fn get_files<P: AsRef<Path>>(path: &P) -> Result<Vec<PathBuf>, Error> {
    let mut vec = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            vec.push(entry_path);
        }
    }
    Ok(vec)
}
