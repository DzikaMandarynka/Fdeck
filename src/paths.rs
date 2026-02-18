#[cfg(debug_assertions)]
pub fn get_save_path<'a>() -> String {
    ".fdeck".to_string()
}
#[cfg(not(debug_assertions))]
pub fn get_save_path<'a>() -> String {
    format!("{}/{}", env!("HOME"), ".fdeck/")
}
