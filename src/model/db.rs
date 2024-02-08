/// db.rs
///
/// Get database connection pool. All access to database from other modules
/// should go through Db struct. Struct Db represents a singleton of connection
/// pool. Users could use this pool to get connection to db. The connections
/// itself could be run synchronously.
use lazy_static::lazy_static;
use std::sync::Mutex;

use super::config::Config;

lazy_static! {
    static ref DB_CONNECTION_POOL: Mutex<Option<mysql::Pool>> = Mutex::new(None);
}

pub struct Db;

impl Db {
    pub fn new() -> Self {
        let mut connection_pool = DB_CONNECTION_POOL.lock().unwrap();
        if connection_pool.is_none() {
            let url = Config::new().get_credential().sql_db_url;
            let pool = mysql::Pool::new(&url[..]).unwrap();
            *connection_pool = Some(pool);
        }
        Self
    }

    pub fn get_connection(&self) -> mysql::PooledConn {
        let connection_pool = DB_CONNECTION_POOL.lock().unwrap();
        (connection_pool).as_ref().unwrap().get_conn().unwrap()
    }
}
