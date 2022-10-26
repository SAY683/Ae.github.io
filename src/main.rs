#![feature(
    arbitrary_enum_discriminant,
    type_alias_impl_trait,
    atomic_from_mut,
    inline_const,
    const_mut_refs,
    associated_type_defaults,
    array_zip
)]
/*
分布式存储
运行前提:>
注意请确保主机Mysql于Redis运行正常;
配置:>
.env:Master节点配置,请确保Master节点设置;
NodSettings:Slave节点配置,不设置/或设置不正常则单机模式;
MysqlPosixSettings/RedisPosixSettings:Mysql与Redis链接配置,Master必须得设置;
错误:>
.env:Master配置问题则默认设置
原理:>
请基本保持有2-3台节点=======
slave1:守护节点(副本)由此处理;
slave2/master:副本存储位置;
slave3/master:副本存储位置;
 */
pub mod beginning;
pub mod node_data;
pub use crate::node_data::{Master, Slave};
pub use anyhow::Result;
use beginning::beginning;
use deadpool::managed::{Manager, Pool};
use futures::executor::block_on;
use lazy_static::lazy_static;
use node_data::SlimeNode;
use once_cell::sync::OnceCell;
use std::future::Future;
use std::pin::Pin;
use tokio::main;
use MysqlOperating::{MysqlServer, SlimeMysql};
use RedisOperating::{RedisServer, SlimeRedis};

///#核心执行
#[main]
pub async fn main() -> Result<()> {
    initialization().await?;
    run().await?;
    shut_down().await?;
    return Ok(());
}
///#初始化
async fn initialization() -> Result<()> {
    beginning(MODEL)
        .await
        .unwrap_or_else(|_| panic!("Initialization Error"));
    return Ok(());
}
///#运行
async fn run() -> Result<()> {
    return Ok(());
}
///#关闭
async fn shut_down() -> Result<()> {
    return Ok(());
}
lazy_static! {
    //ping mysql联通性返回
    pub static ref MYSQL_VERSION: Result<bool> = {
        if MODEL {
            let mut x= block_on(Master::conn(&Master::get_pool(&TEST_MYSQL.get().unwrap().handle()?),))?;
            let r=block_on(Master::ping(&mut x)).is_ok();
            block_on(x.disconnect())?;
            Ok(r)
        } else {
            let mut x= block_on(Master::conn(&Master::get_pool(&MYSQL.get().unwrap().handle()?),))?;
            let r=block_on(Master::ping(&mut x)).is_ok();
            block_on(x.disconnect())?;
            Ok(r)
        }
    };
    //ping redis联通性返回
    pub static ref REDIS_VERSION: Result<bool>={
        if MODEL{
            Ok(Master::ping_lot(&Master::get_redis(&TEST_REDIS.get().unwrap().handle()?)?)?)
        }else {
            Ok(Master::ping_lot(&Master::get_redis(&REDIS.get().unwrap().handle()?)?)?)
        }
    };
}
//#相关配置
pub static MASTER: OnceCell<Master> = OnceCell::new();
pub static SLAVE: OnceCell<Slave> = OnceCell::new();
pub static MYSQL: OnceCell<SlimeMysql> = OnceCell::new();
pub static REDIS: OnceCell<SlimeRedis> = OnceCell::new();
///#测试用配置|默认
pub static TEST_MASTER: OnceCell<Master> = OnceCell::new();
pub static TEST_SLAVE: OnceCell<Slave> = OnceCell::new();
pub static TEST_MYSQL: OnceCell<SlimeMysql> = OnceCell::new();
pub static TEST_REDIS: OnceCell<SlimeRedis> = OnceCell::new();
///#测试模式true/执行false
pub const MODEL: bool = true;
///#节点文件配置
pub const NODE_INIT: [&str; 2] = [".", "NodeSettings.json"];
///#Mysql数据端口配置
pub const MYSQL_PORT_INIT: [&str; 2] = [".", "MysqlPortSettings.json"];
///#Redis数据端口配置
pub const REDIS_PORT_INIT: [&str; 2] = [".", "RedisPortSettings.json"];
///#异步闭包
pub struct AsyncDriver<'life, Rx: Sized>(
    pub Pin<Box<dyn Future<Output = Result<Rx>> + Send + Sync + 'life>>,
);
///#异步池[async_trait]实现注意
pub struct AsynchronousPool<G: Sized + Manager>(pub Pool<G>);
