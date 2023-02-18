use rwgps::{config::UserConfig};
use rwgps_types::Route;

use crate::dirs::CONFIG_DIRPATH;

const CONFIG_FILENAME: &'static str = "rwgps_auth.toml";

pub fn load_user_config() -> Result<Option<UserConfig>, anyhow::Error> {
    let config_filepath = CONFIG_DIRPATH.join(CONFIG_FILENAME);

    match config_filepath.exists() {
        true => Ok(Some(toml::from_str(&String::from_utf8(std::fs::read(
            &config_filepath,
        )?)?)?)),
        false => Ok(None),
    }
}

pub fn persist_user_config(config: &UserConfig) -> Result<(), anyhow::Error> {
    let config_filepath = CONFIG_DIRPATH.join(CONFIG_FILENAME);

    std::fs::write(config_filepath, toml::to_string(config)?)?;

    Ok(())
}

pub fn load_routes() -> Result<Vec<Route>, anyhow::Error> {
    let routes_filepath = CONFIG_DIRPATH.join("rwgps_routes.json");

    let data = std::fs::read(routes_filepath)?;
    Ok(serde_json::from_slice(&data)?)
}

pub fn persist_routes(routes: &Vec<rwgps_types::Route>) -> Result<(), anyhow::Error> {
    let routes_filepath = CONFIG_DIRPATH.join("rwgps_routes.json");

    std::fs::write(routes_filepath, serde_json::to_vec(routes)?)?;

    Ok(())
}

pub fn load_trips() -> Result<Vec<rwgps_types::Trip>, anyhow::Error> {
    let trips_filepath = CONFIG_DIRPATH.join("rwgps_trips.json");

    let data = std::fs::read(trips_filepath)?;
    Ok(serde_json::from_slice(&data)?)
}

pub fn persist_trips(trips: &Vec<rwgps_types::Trip>) -> Result<(), anyhow::Error> {
    let trips_filepath = CONFIG_DIRPATH.join("rwgps_trips.json");

    std::fs::write(trips_filepath, serde_json::to_vec(trips)?)?;

    Ok(())
}
