/*
Mysql操作
MYSQL在本系统中将担任分布文件聚合处理
 */
use anyhow::Result;
use async_trait::async_trait;
use core::fmt::Debug;
use mysql_async::prelude::{Query, Queryable};
use mysql_async::{Conn as AsyncConn, Pool as AsyncPool};
use rbatis::{crud, impl_select, Rbatis};
use rbdc::datetime::FastDateTime;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::adapter::Urn;
use uuid::Uuid;

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
    fn uid() -> String {
        return Urn::from_uuid(Uuid::new_v4()).to_string();
    }
}
impl MysqlServer for SlimeMysql {}
///#查询操作
#[async_trait]
pub trait MysqlOrm {
    ///#生成orm_get async fn orm_get(e: &str) -> Result<Rbatis>
    async fn orm_get(e: &str) -> Result<Rbatis> {
        let rb = Rbatis::new();
        rb.init(MysqlDriver {}, e)?;
        return Ok(rb);
    }
    type Object;
    ///#查询全部
    async fn orm_select() -> Result<Self::Object>;
    type Data;
    ///#节点计算
    async fn orm_database_node(&self) -> Result<Self::Data>;
    ///#插入
    async fn orm_insert(_: Self::Object) -> Result<()>;
    type DataTable;
    ///#更新
    async fn orm_update(_: Self::DataTable, _: String) -> Result<Self::DataTable>;
    ///#删除
    async fn orm_remove(_: String) -> Result<()>;
}
///#MysqlHdfs
#[async_trait]
pub trait MysqlHdfsDatabaseDriver {
}
///#默认数据表
#[derive(Hash, Clone, Debug, Serialize, Deserialize)]
pub struct AeExam {
    //其他时间表接口 Option特殊情况不用写
    pub id: Option<String>,
    //分布式虚拟文件名称
    pub name: String,
    //hash文件验证值
    pub hash: Option<String>,
    //存储位置jsonNode
    pub location: Option<String>,
    //时间记录
    pub time: Option<FastDateTime>,
}
impl MysqlServer for AeExam {}
//依据实现
crud!(AeExam {});
//查询id
impl_select!(AeExam {select_id(id:&str)=>"where id = #{name}"});
//查询名称
impl_select!(AeExam{select_name(name:&str)=>"where name = #{name}"});
//更新
///#错误
#[derive(Debug, Error)]
pub enum AeMysqlError<'life> {
    ///初始化异常
    #[error("INITIALIZATION_EXCEPTION")]
    Initialization,
    ///链接异常
    #[error("INITIALIZATION[IP:{ip:?}|Error:{error:?}]")]
    Link { ip: String, error: String },
    ///#列表错误
    #[error("AListOfErrors{a:?}|{b:?}")]
    AListOfErrors { a: i64, b: i64 },
    ///未知因此
    #[error("UNKNOWN-SO:{0}")]
    Unknown(&'life str),
}

///Ae_Exam创建语句
pub const AE_EXAM: &str = r"
create table if not exists ae_exam
(
	id varchar(1989) not null,
	name varchar(1989) not null,
	hash text null,
	location longtext null,
	time datetime null
)engine=InnoDB,charset=utf8mb4;
create index ae_exam_id_index
	on exam (id);
create unique index ae_exam_name_uindex
	on exam (name);
alter table ae_exam
	add constraint ae_exam_pk
		primary key (name);
";
