use std::path::PathBuf;
#[cfg(debug_assertions)]
pub fn get_save_path() -> PathBuf {
    PathBuf::from(".fdeck/")
}
#[cfg(not(debug_assertions))]
pub fn get_save_path() -> PathBuf {
    PathBuf::from(format!("{}/{}", env!("HOME"), ".fdeck/"))
}
