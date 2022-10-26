/*
Redis操作
 */
use anyhow::Result;
use async_trait::async_trait;
use deadpool_redis::redis::{Client, ConnectionLike};
use deadpool_redis::{Config as PoolConfig, Pool as PoolC, Runtime};
use r2d2_redis::r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use serde::{Deserialize, Serialize};
///#Redis_Ulr
#[derive(Debug, Serialize, Deserialize)]
pub struct SlimeRedis {
    pub name: Option<String>,
    pub password: Option<String>,
    pub ip: String,
    pub port: String,
    pub database: String,
}
impl Default for SlimeRedis {
    ///#Redis_Ulr测试用
    fn default() -> Self {
        return SlimeRedis {
            name: None,
            password: None,
            ip: "127.0.0.1".to_string(),
            port: "6379".to_string(),
            database: "".to_string(),
        };
    }
}
///#RedisServer
///#已Macro
#[async_trait]
pub trait RedisServer {
    ///#fn get_redis_r2d2(e:&str)-> Result<Pool<RedisConnectionManager>>
    fn get_redis_r2d2(e: &str) -> Result<Pool<RedisConnectionManager>> {
        return Ok(Pool::new(RedisConnectionManager::new(e)?)?);
    }
    ///#fn get_redis_r2d2(e: &str)->Result<Client>
    fn get_redis(e: &str) -> Result<Client> {
        return Ok(Client::open(e)?);
    }
    ///#fn ping fn ping_lot(e: &Client) -> Result<Connection>
    fn ping_lot(e: &Client) -> Result<bool> {
        return Ok(e.get_connection()?.is_open());
    }
}
///#查询池
#[async_trait]
pub trait RedisServerPoll {
    ///#database get Config
    fn get_redis_poll(e: &str) -> PoolConfig {
        return PoolConfig::from_url(e);
    }
    ///#database pool
    fn get_redis_pool(e: PoolConfig) -> Result<PoolC> {
        return Ok(e.create_pool(Some(Runtime::Tokio1))?);
    }
}
