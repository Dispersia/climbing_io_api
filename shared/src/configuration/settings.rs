use coi::Inject;
use config::FileFormat;
use serde::Deserialize;

#[derive(Inject, Debug, Deserialize)]
#[coi(provides pub Settings with load_config())]
pub struct Settings {
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub connection_string: String,
    pub cert_directory: String,
}

fn load_config() -> Settings {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::from_str(SETTINGS_FILE, FileFormat::Toml))
        .unwrap()
        .merge(config::File::from_str(ENV_SETTINGS_FILE, FileFormat::Toml))
        .unwrap()
        .merge(config::Environment::with_prefix("APP").separator("__"))
        .unwrap();

    settings.try_into().unwrap()
}

const SETTINGS_FILE: &'static str = include_str!("../../settings.toml");

#[cfg(feature = "dev")]
const ENV_SETTINGS_FILE: &'static str = include_str!("../../settings.dev.toml");

#[cfg(feature = "prod")]
const ENV_SETTINGS_FILE: &'static str = include_str!("../../settings.prod.toml");
