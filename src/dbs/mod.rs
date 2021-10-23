pub mod dbr;
pub mod dbp;

use bb8::{Pool, PooledConnection, RunError};
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::RedisConnectionManager;
use redis::{RedisError, ToRedisArgs, cmd};
use tokio_postgres::{NoTls, RowStream};
use sha3::{Digest, Sha3_224};
use futures::stream::{TryStreamExt};
use async_trait::async_trait;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::mode::ExtJson;


#[async_trait]
pub trait FromRaw {
    async fn convert(raw: RowStream) -> Result<Vec<Self>, String> where Self:Sized;
}

pub struct DBS {
    pg: Pool<PostgresConnectionManager<tokio_postgres::NoTls>>,
    rd: Pool<RedisConnectionManager>
}

impl DBS {
    pub async fn new(pg_addr: &str, rd_addr: &str) -> Self {
        let (pg, rd) = tokio::join!(dbp::posg_manager(pg_addr), dbr::redis_manager(rd_addr));
        DBS {
            pg: pg,
            rd: rd
        }
    }

    pub async fn get_rd_conn<'a>(&'a self) -> Result<PooledConnection<'a, RedisConnectionManager>, RunError<RedisError>>{
        let conn = self.rd.get_owned().await;
        conn
    }

    pub async fn get_pg_conn<'a>(&'a self) -> Result<PooledConnection<'a, PostgresConnectionManager<NoTls>>, RunError<tokio_postgres::Error>> {
        let conn = self.pg.get_owned().await;
        conn
    }

    pub async fn get_conn<'a>(&'a self) -> Result<(PooledConnection<'a, PostgresConnectionManager<NoTls>>, PooledConnection<'a, RedisConnectionManager>), String> {
        let (r1, r2) = tokio::join!(self.get_pg_conn(), self.get_rd_conn());
        let c1 = r1.map_err(|e| {e.to_string()})?;
        let c2 = r2.map_err(|e| {e.to_string()})?;
        Ok((c1, c2))
    }
    
    pub fn gen_key(&self, s: &str, params: &Vec<String>) -> String {
        let mut hasher = Sha3_224::new();
        hasher.update(s);
        hasher.update(params.join(""));
        format!("{:x}", hasher.finalize())
    }
    
    pub async fn cache_query<T>(&self, statement: &str, params: Vec<String>) -> Result<Vec<T>, String>
    where T: FromRaw + ToRedisArgs + Serialize + DeserializeOwned + Clone {
        let (pg, mut rd) = self.get_conn().await.map_err(|e| {e})?;
        let key = self.gen_key(statement, &params);
        let cache_result: Result<String, RedisError> = cmd("GET").arg(key.clone()).query_async(&mut *rd).await;
        match cache_result {
            Ok(r) => {
                println!("redis val = {}", r);
                let mut data: Vec<T> = Vec::new();
                data = data.ext_deserialize(&r)?;
                Ok(data)
            },
            Err(_) => {
                println!("get from pgsq");
                let raw = pg.query_raw(statement, params).await.map_err(|e| {e.to_string()})?;
                let data = FromRaw::convert(raw).await?;
                let v_str = data.ext_serialize();
                let srd: Result<String, RedisError> = cmd("SET").arg(key).arg(&v_str).query_async(&mut *rd).await;
                srd.map_err(|e| {e.to_string()})?;
                Ok(data)
            }
        }
    }

    pub fn cache_update() {

    }
}