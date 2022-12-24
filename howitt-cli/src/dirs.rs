use std::path::PathBuf;

const APP_NAME: &'static str = "howitt";

pub fn config_dirpath() -> PathBuf {
    dirs::config_dir().unwrap().join(APP_NAME)
}