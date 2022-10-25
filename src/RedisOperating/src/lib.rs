/*
Redis操作
 */
use anyhow::Result;
use async_trait::async_trait;
use deadpool_redis::{Config as PoolConfig, Pool as PoolC, Runtime};
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
#[async_trait]
pub trait RedisServerPoll {
    ///#database get Config
    fn get(e: &str) -> PoolConfig {
        return PoolConfig::from_url(e);
    }
    ///#database pool
    fn get_pool(e: PoolConfig) -> Result<PoolC> {
        return Ok(e.create_pool(Some(Runtime::Tokio1))?);
    }
}
