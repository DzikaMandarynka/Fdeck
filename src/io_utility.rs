use crate::paths;
use std::{fs, io::Error, path::Path};

pub fn is_save_present() -> bool {
    Path::new(&paths::get_save_path()).exists()
}

pub fn create_save() -> Result<(), Error> {
    fs::create_dir(&paths::get_save_path())
}

pub fn overwrite_dir(path: &Path) -> Result<(), Error> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir(path)
}
