/*
Mysql操作
MYSQL在本系统中将担任分布文件聚合处理
 */
use anyhow::Result;
use async_trait::async_trait;
use core::fmt::Debug;
use mysql_async::prelude::{Query, Queryable};
use mysql_async::{Conn as AsyncConn, Pool as AsyncPool};
use rbatis::{crud, Rbatis};
use rbdc::datetime::FastDateTime;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};
use thiserror::Error;
///#Mysql_Ulr
#[derive(Debug, Serialize, Deserialize)]
pub struct SlimeMysql {
    pub name: String,
    pub password: String,
    pub host: String,
    pub database: String,
}
impl Default for SlimeMysql {
    ///#Mysql_Ulr测试用
    fn default() -> Self {
        return SlimeMysql {
            name: "root".to_string(),
            database: "cabinet".to_string(),
            password: "683S@y683".to_string(),
            host: "localhost".to_string(),
        };
    }
}
///#Mysql_Server
///#以Macro
///#基本处理
#[async_trait]
pub trait MysqlServer {
    ///#async_mysql get fn get(e: &str) -> AsyncPool
    fn get_pool(e: &str) -> AsyncPool {
        return AsyncPool::new(e);
    }
    ///#async_mysql pool async fn conn(e: &AsyncPool) -> Result<Conn>
    async fn conn(e: &AsyncPool) -> Result<AsyncConn> {
        return Ok(e.get_conn().await?);
    }
    ///#AsyncPool move AsyncPool to disconnect<断开>
    ///#async_mysql pool async fn async fn quote(e: &str, r: &AsyncPool) -> Result<()>
    async fn quote(e: &str, r: AsyncPool) -> Result<()> {
        e.ignore(&r).await?;
        r.disconnect().await?;
        return Ok(());
    }
    ///#async_mysql ping async fn ping(r:&mut AsyncConn) -> Result<(u16, u16, u16)>
    async fn ping(r: &mut AsyncConn) -> Result<(u16, u16, u16)> {
        r.ping().await?;
        return Ok(r.server_version());
    }
}
///#查询操作
#[async_trait]
pub trait MysqlOrm {
    ///#生成orm_get async fn orm_get(e: &str) -> Result<Rbatis>
    async fn orm_get(e: &str) -> Result<Rbatis> {
        let rb = Rbatis::new();
        rb.init(MysqlDriver {}, e)?;
        return Ok(rb);
    }
    type Data;
    ///#节点计算
    async fn orm_database_node(&self) -> Result<Self::Data>;
}
///#默认数据表
#[derive(Hash, Clone, Debug, Serialize, Deserialize)]
pub struct AeExam {
    //其他时间表接口
    pub id: usize,
    //分布式虚拟文件名称
    pub name: String,
    //hash文件验证值
    pub hash: Option<String>,
    //存储位置jsonNode
    pub location: Option<String>,
    //时间记录
    pub time: Option<FastDateTime>,
}
//依据实现
crud!(AeExam {});
///Ae_Exam创建语句
pub const AE_EXAM: &str = r"
create table if not exists ae_exam
(
	id bigint(25) not null,
	name varchar(1989) not null,
	hash text null,
	location longtext null,
	time datetime null
);
create index exam_id_index
	on exam (id);
create unique index exam_name_uindex
	on exam (name);
alter table exam
	add constraint exam_pk
		primary key (name);
";
///#错误
#[derive(Debug, Error)]
pub enum AeMysqlError<'life> {
    ///初始化异常
    #[error("INITIALIZATION_EXCEPTION")]
    Initialization,
    ///链接异常
    #[error("INITIALIZATION[IP:{ip:?}|Error:{error:?}]")]
    Link { ip: String, error: String },
    ///未知因此
    #[error("UNKNOWN-SO:{0}")]
    Unknown(&'life str),
}
