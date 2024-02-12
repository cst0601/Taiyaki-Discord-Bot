/// config.rs
///
/// Stores configuration files and secrets for discord API and database.
use serde::{Serialize, Deserialize};
use std::{
    sync::OnceLock,
    fs::File,
    io::BufReader,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigData {
    credential_data: CredentialData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CredentialData {
    pub discord_token: String,
    pub sql_db_url: String,
}

fn read_credentials() -> Result<CredentialData, Box<dyn std::error::Error>> {
    let file = File::open("credentials/secrets.json")?;
    let reader = BufReader::new(file);

    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

pub fn get_config_data() -> &'static ConfigData {
    static CONFIG_DATA: OnceLock<ConfigData> = OnceLock::new();
    CONFIG_DATA.get_or_init(|| {
        ConfigData {
            credential_data: read_credentials().expect("Error reading credentials"),
        }
    })
}

pub fn get_credential_data() -> &'static CredentialData {
    &get_config_data().credential_data
}
