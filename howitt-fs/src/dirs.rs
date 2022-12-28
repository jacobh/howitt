use std::path::PathBuf;

const APP_NAME: &'static str = "howitt";

lazy_static::lazy_static! {
    pub static ref CONFIG_DIRPATH: PathBuf = {
        let dirpath = dirs::config_dir().unwrap().join(APP_NAME);
        std::fs::create_dir_all(&dirpath).unwrap();
        dirpath
    };
}
