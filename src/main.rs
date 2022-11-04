#![feature(
arbitrary_enum_discriminant,
type_alias_impl_trait,
atomic_from_mut,
inline_const,
const_mut_refs,
associated_type_defaults,
array_zip,
box_syntax,
let_chains,
unboxed_closures,
async_closure,
type_ascription,
never_type,
)]
#![feature(label_break_value)]

pub mod beginning;
mod database_link;
mod hdfs_service;
pub mod node_data;

use crate::beginning::beginning_log;
pub use crate::node_data::{Master, Slave};
pub use anyhow::Result;
use async_backtrace::framed;
use beginning::beginning;
use database_link::mysql::StorageLocation;
use deadpool::managed::{Manager, Pool};
use deadpool_redis::redis::Client;
use deluge::Iter;
use futures::executor::block_on;
use lazy_static::lazy_static;
use node_data::SlimeNode;
use once_cell::sync::OnceCell;
use r2d2_redis::RedisConnectionManager;
use rbatis::Rbatis;
use std::future::Future;
use std::net::UdpSocket;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use tokio::main;
use FileOperations::program_file_setup::{ApplicationSettings, Setting};
use HdfsService::SAR;
use MysqlOperating::{MysqlServer, SlimeMysql};
use RedisOperating::{RedisServer, SlimeRedis};

///#核心执行
#[main]
pub async fn main() -> Result<()> {
	initialization().await.unwrap_or_else(|x| panic!("{}", x));
	run().await.unwrap_or_else(|x| panic!("{}", x));
	shut_down().await.unwrap_or_else(|x| panic!("{}", x));
	return Ok(());
}

///#初始化
#[framed]
async fn initialization() -> Result<()> {
	let x = ApplicationSettings::new()?.handle()?;
	SETTINGS.get_or_init(|| x);
	beginning(SETTINGS.get().unwrap().default).await?;
	beginning_log(SETTINGS.get().unwrap().logs)?;
	return Ok(());
}

///#运行
#[framed]
async fn run() -> Result<()> {
	return Ok(());
}

///#关闭
#[framed]
async fn shut_down() -> Result<()> {
	return Ok(());
}
lazy_static! {
    //ping mysql联通性返回
    pub static ref MYSQL_VERSION: Result<bool> = {
        if SETTINGS.get().unwrap().default {
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
        if SETTINGS.get().unwrap().default{
            Ok(Master::ping_lot(&Master::get_redis(&TEST_REDIS.get().unwrap().handle()?)?)?)
        }else {
            Ok(Master::ping_lot(&Master::get_redis(&REDIS.get().unwrap().handle()?)?)?)
        }
    };
    ///#链接池r2d2_redis
    pub static ref REDIS_DIR:Result<RedisConnectionManager>={
        Ok(RedisConnectionManager::new(TEST_REDIS.get().unwrap().handle()?)?)
    };
    ///#本机ip
    pub static ref LOCAL_IP: Result<String>={
        let x = UdpSocket::bind("0.0.0.0:0")?;
        x.connect("8.8.8.8:80")?;
        return Ok(x.local_addr()?.ip().to_string());
    };
    //#链接池mysql
    pub static ref MYSQL_DIR_INIT:Result<Rbatis>={
        Ok(block_on(StorageLocation::get_mysql::<Master>())?)
    };
    //#链接池deadpool_redis
    pub static ref REDIS_DIR_INIT:Result<Client>={
        Ok(SlimeRedis::get_redis(&if SETTINGS.get().unwrap().default {
                TEST_REDIS.get().unwrap().handle()?
            } else {
                REDIS.get().unwrap().handle()?
            })?)
    };
    //随机ID
    pub static ref ID:String={
        Master::uid()
    };
    //LOG路径
    pub static ref LOG_DIR:Result<PathBuf>={
        let mut x = Master::new()?.logs;
        x.push("Ae_Logs.log");
        return Ok(x);
    };
    //通信密钥
    pub static ref COMMUNICATION_PME:Result<SAR>={
        return Ok(SAR::new()?);
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
///#系统设置
pub static SETTINGS: OnceCell<Setting> = OnceCell::new();
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

///#异步迭代器[deluge]实现
pub struct AsynchronousIterator<G: Sized + IntoIterator>(Iter<G>);

///#节点模式 true集群 默认本机
pub static THE_NODE_MODEL: AtomicBool = AtomicBool::new(false);
///#是否是 master节点 默认是master
pub static MASTER_MODEL: AtomicBool = AtomicBool::new(true);
