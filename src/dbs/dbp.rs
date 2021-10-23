use std::str::FromStr;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{config, tls};

pub async fn posg_manager(addr: &str) -> Pool<PostgresConnectionManager<tokio_postgres::NoTls>>  {
    let cfg = config::Config::from_str(addr).unwrap();
    let manager = PostgresConnectionManager::new(cfg, tls::NoTls);
    let pool = Pool::builder().build(manager).await.unwrap();
    pool
}

//pg sql statement
pub const SQL_QUERY_USERS: &str = "SELECT id, userid, uname FROM myschema.user";
const SQL_QUERY_USERS_BY_ID: &str = "SELECT id, userid, uname FROM myschema.user WHERE id = $1";

// pub async fn get_users(c: tokio_postgres::Client, params: Vec<String>) {
//     let result = c.query_raw(SQL_QUERY_USERS, params).await;
// }

// pub fn gen_key(s: &str, params: Vec<String>) -> String {
//     let mut hasher = Sha3_224::new();
//     hasher.update(s);
//     hasher.update(params.join(""));
//     format!("{:x}", hasher.finalize())
// }