use bb8::Pool;
use bb8_redis::{
    bb8, RedisConnectionManager
};

pub async fn redis_manager(addr: &str) -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new(addr).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    pool
}
