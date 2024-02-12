/// db.rs
///
/// Get database connection pool. All access to database from other modules
/// should go through get_connection function. This module represents a
/// singleton of pool. Users could use this pool to get connection to db. The
/// connections itself could be run synchronously.
use std::sync::OnceLock;

use super::config;

pub fn get_connection() -> mysql::PooledConn {
    static DB_CONNECTION_POOL: OnceLock<mysql::Pool> = OnceLock::new();
    DB_CONNECTION_POOL.get_or_init(|| {
        let url = config::get_credential_data().sql_db_url.clone();
        mysql::Pool::new(&url[..]).unwrap()
    }).get_conn().unwrap()
}