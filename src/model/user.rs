/// user.rs
///
/// User data related database operations belongs here.
use mysql::{ Error, params, prelude::Queryable };
use log::{ warn, debug };

use crate::model::db;

#[derive(Debug)]
pub struct User {
    discord_id: u64,
    pub username: String,
    pub taiyaki_count: u32,
    pub level: u32,
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
    let mut conn = db::get_connection();

    conn.exec_drop("
        INSERT INTO User (discord_id, username) VALUES
        (:discord_id, :username)",
        params! {
            "discord_id" => user.discord_id,
            "username" => user.username.clone(),
        })?;

    Ok(())
}

pub fn add_taiyaki_by_id(discord_id: u64) {
    let _ = db::get_connection().exec_drop("
            UPDATE User
            SET taiyaki_count = taiyaki_count + 1
            WHERE discord_id = :discord_id
        ",
        params!{
            "discord_id" => discord_id,
        }
    ).inspect_err(|e| {
        warn!("Error connecting to database: {}", e);
    });
}

pub fn get_user_by_id(discord_id: u64) -> Result<Option<User>, Error> {
    let mut conn = db::get_connection();

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

pub fn get_leaderboard() -> Result<Vec<User>, Error> {
    let mut conn = db::get_connection();

    let users = conn.query_map(
        "SELECT * FROM User ORDER BY taiyaki_count DESC LIMIT 10",
        |(discord_id, username, taiyaki_count, level)|
            User { discord_id, username, taiyaki_count, level }
    );

    users
}
