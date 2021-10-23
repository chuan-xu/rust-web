use futures::stream::TryStreamExt;
use redis::{RedisWrite, ToRedisArgs};
use tokio_postgres::RowStream;
use serde::{Deserialize, Serialize};
use serde_json;
use async_trait::async_trait;
use crate::dbs::FromRaw;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    id: u32,
    uid: String,
    uname: String,
}

impl User {
    fn new(id: u32, uid: String, uname: String) -> Self {
        User { id, uid, uname }
    }
}

impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let v = serde_json::to_string(self).unwrap();
        out.write_arg(v.as_bytes())
    }
}

#[async_trait]
impl FromRaw for User {
    async fn convert(raws: RowStream) -> Result<Vec<Self>, String> {
        let mut v: Vec<User> = Vec::new();
        raws.try_for_each(|r| {
            let id: i32 = r.get("id");
            let u = User::new(id as u32, r.get("userid"), r.get("uname"));
            v.push(u);
            futures::future::ready(Ok(()))
        }).await.map_err(|e|{e.to_string()})?;
        Ok(v)
    }
}