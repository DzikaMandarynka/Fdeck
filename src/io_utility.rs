use crate::constants::SAVE_DIRECTORY;
use std::{fs, io::Error, path::Path};

pub fn is_save_present() -> bool {
    Path::new(SAVE_DIRECTORY).exists()
}

pub fn create_save() -> Result<(), Error> {
    fs::create_dir(SAVE_DIRECTORY)
}

pub fn overwrite_dir(path: &Path) -> Result<(), Error> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir(path)
}
