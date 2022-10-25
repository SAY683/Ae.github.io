/*
Mysql操作
 */
use anyhow::Result;
use async_trait::async_trait;
use mysql_async::prelude::{Query, Queryable};
use mysql_async::{Conn as AsyncConn, Pool as AsyncPool};
use rbatis::Rbatis;
use rbdc::datetime::FastDateTime;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};
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
}
///#默认数据表
#[derive(Hash, Clone, Debug, Serialize, Deserialize)]
pub struct AeExam {
    //分布式虚拟文件名称
    pub name: String,
    //hash文件验证
    pub hash: String,
    //时间记录
    pub time: FastDateTime,
}
