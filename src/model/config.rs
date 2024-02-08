/// config.rs
///
/// Stores configuration files and secrets for discord API and database.
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use std::{
    sync::Mutex,
    fs::File,
    io::BufReader,
};

lazy_static! {
    static ref CONFIG_DATA: Mutex<Option<ConfigData>> = Mutex::new(None);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigData {
    credential_data: CredentialData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CredentialData {
    pub discord_token: String,
    pub sql_db_url: String,
}

// access point
pub struct Config;

impl Config {
    pub fn new() -> Self {
        let mut config_data = CONFIG_DATA.lock().unwrap();
        if config_data.is_none() {
            *config_data = Some(ConfigData {
                credential_data: read_credentials().expect("Error reading credentials"),
            });
        }
        Self
    }

    pub fn get_credential(&self) -> CredentialData {
        let config_data = CONFIG_DATA.lock().unwrap();
        config_data.clone().unwrap().credential_data
    }
}

pub fn read_credentials() -> Result<CredentialData, Box<dyn std::error::Error>> {
    let file = File::open("credentials/secrets.json")?;
    let reader = BufReader::new(file);

    let data = serde_json::from_reader(reader)?;

    Ok(data)
}
