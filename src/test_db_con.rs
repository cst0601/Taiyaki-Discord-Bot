// sql server test
use mysql::*;
use mysql::prelude::*;

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