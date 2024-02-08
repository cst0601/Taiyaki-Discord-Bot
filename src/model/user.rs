/// user.rs
///
/// User data related database operations belongs here.
use mysql::*;
use mysql::prelude::*;
use log::debug;

use crate::model::db::Db;

#[derive(Debug)]
pub struct User {
    discord_id: u64,
    username: String,
    taiyaki_count: u32,
    level: u32,
}

impl User {
    pub fn new(discord_id: u64, username: String) -> Self {
        let user = User {
            discord_id,
            username,
            taiyaki_count: 0,
            level: 1,
        };
        let _ = create_user(&user);
        user
    }
}

/// Creates a new user
///
fn create_user(user: &User) ->
    std::result::Result<(), Box<dyn std::error::Error>>
{
    debug!("Connecting to db to create user...");
    let mut conn = Db::new().get_connection();

    conn.exec_drop("
        INSERT INTO User (discord_id, username) VALUES
        (:discord_id, :username)",
        params! {
            "discord_id" => user.discord_id,
            "username" => user.username.clone(),
        })?;

    Ok(())
}

fn get_user_by_id(discord_id: u64) -> Result<Option<User>, Error> {
    let mut conn = Db::new().get_connection();

    let user = conn
        .exec_first(
            "SELECT * FROM User WHERE discord_id=:discord_id",
            params!{
                "discord_id" => discord_id,
        })
        .map(|row| {
            row.map(|(discord_id, username, taiyaki_count, level)| User {
                discord_id, username, taiyaki_count, level,
            })
        });

    user
}
