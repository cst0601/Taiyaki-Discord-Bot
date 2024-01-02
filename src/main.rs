use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use log::{info, warn};

use serde::Deserialize;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use taiyaki_maker::handlers::{ping_handler, handler};
use taiyaki_maker::service::create_services;

// sql server test
use mysql::*;
use mysql::prelude::*;

#[derive(Deserialize, Debug)]
struct Credential {
    discord_token: String,
    sql_db_url: String,
}

#[derive(Debug)]
struct User {
    user_id: u32,
    discord_id: String,
    user_name: String,
    taiyaki_count: u32,
    level: u32,
}

fn test_db_connect(url: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to db...");
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let selected_users = conn.query_map(
        "SELECT * FROM USER",
        |(user_id, discord_id, user_name, taiyaki_count, level)| {
            User { user_id, discord_id, user_name, taiyaki_count, level }
        },
    )?;
    for user in selected_users {
        println!("{:?}", user);
    }

    Ok(())
}

fn read_credentials() -> Result<Credential, Box<dyn std::error::Error>> {
    let file = File::open("credentials/secret.json")?;
    let reader = BufReader::new(file);

    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    info!("Reading credentials and setting up connection...");
    let secrets = read_credentials().expect("Error reading credentials.");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    //test_db_connect(&secrets.sql_db_url).unwrap();

    let mut client: Client =
        Client::builder(&secrets.discord_token, intents)
        .event_handler(handler::Handler)
        .framework(create_services())
        .await
        .expect("Err creating client");
    if let Err(e) = client.start().await {
        warn!("Client error: {:?}", e);
    }
}
