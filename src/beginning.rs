use std::sync::atomic::Ordering;
use crate::{Master, Result, Slave, SlimeMysql, SlimeNode, SlimeRedis, LOCAL_IP, MASTER, MYSQL, MYSQL_VERSION, REDIS, REDIS_VERSION, SLAVE, TEST_MASTER, TEST_MYSQL, TEST_REDIS, TEST_SLAVE, THE_NODE_MODEL, LOG_DIR, SETTINGS};
use compact_str::CompactString;
use log::{log, Level};
use FileOperations::local_data;
use FileOperations::local_data::FileOperation;
use MysqlOperating::{MysqlServer, AE_EXAM};
use crate::hdfs_service::HdfsManager;
use ftlog::writer::file_split::Period;
use ftlog::{LevelFilter, LogBuilder};

///初始
pub async fn beginning(e: bool) -> Result<()> {
	if e {
		log!(Level::Info, "Testing:<{}>MODE", e);
		testing();
		server_setting(e)
			.await
			.unwrap_or_else(|e| log!(Level::Debug, "Data Is Error[{}]", e));
	} else {
		log!(Level::Info, "Execute:<{}>MODE", !e);
		data_ing()?;
		server_setting(e)
			.await
			.unwrap_or_else(|e| log!(Level::Debug, "Data Is Error[{}]", e));
	}
	file_init(e)?;
	return Ok(());
}

///#数据链接初始
async fn server_setting(e: bool) -> Result<()> {
	//初始
	HdfsManager::new()?.handle()?;
	if THE_NODE_MODEL.load(Ordering::Acquire) {
		match ping().await? {
			(x, y) if x == y && x == true => {
				//初始数据表
				Master::quote(
					AE_EXAM,
					Master::get_pool(&if e {
						TEST_MYSQL.get().unwrap().handle()?
					} else {
						MYSQL.get().unwrap().handle()?
					}),
				)
					.await?;
			}
			_ => {
				panic!("Basic configuration error")
			}
		};
	}
	return Ok(());
}

///#文件初始[crate::beginning::master_init]
fn file_init(e: bool) -> Result<()> {
	let x = if e {
		(TEST_SLAVE.get().unwrap(), TEST_MASTER.get().unwrap())
	} else {
		(SLAVE.get().unwrap(), MASTER.get().unwrap())
	};
	if x.0.slave.len() == 0 || LOCAL_IP.as_ref().unwrap() == &x.1.local.ip().to_string() {
		master_init(e)?;
	} else {
		eprintln!("Node Tis Not Master");
	}
	return Ok(());
}

///#服务器链接测试 Mysql|Redis|
pub async fn ping() -> Result<(bool, bool)> {
	return Ok((
		*MYSQL_VERSION.as_ref().unwrap_or_else(|_| {
			log!(Level::Debug, "Master_Mysql_Error");
			&false
		}),
		*REDIS_VERSION.as_ref().unwrap_or_else(|_| {
			log!(Level::Debug, "Master_Redis_Error");
			&false
		}),
	));
}

///#数据初始
fn data_ing() -> Result<()> {
	let x = Master::new()?;
	MASTER.get_or_init(|| x);
	let x = Slave::new()?;
	SLAVE.get_or_init(|| x);
	let x = SlimeMysql::new()?;
	MYSQL.get_or_init(|| x);
	let x = SlimeRedis::new()?;
	REDIS.get_or_init(|| x);
	return Ok(());
}

///#数据测试
fn testing() {
	TEST_MASTER.get_or_init(|| Master::default());
	TEST_SLAVE.get_or_init(|| Slave::default());
	TEST_MYSQL.get_or_init(|| SlimeMysql::default());
	TEST_REDIS.get_or_init(|| SlimeRedis::default());
}

///#master初始
fn master_init(e: bool) -> Result<()> {
	let Master {
		local: _,
		hdfs,
		logs,
	} = if e {
		TEST_MASTER.get().unwrap()
	} else {
		MASTER.get().unwrap()
	};
	local_data::LocalFileOperations([local_data::FileOperations::Establish([
		(
			CompactString::new(hdfs.to_str().unwrap()),
			Vec::<CompactString>::new(),
		),
		(
			CompactString::new(logs.to_str().unwrap()),
			Vec::<CompactString>::new(),
		),
	])])
		.run()?;
	return Ok(());
}

pub fn beginning_log(e: bool) -> Result<()> {
	if e {
		let x = LOG_DIR.as_ref().unwrap();
		LogBuilder::new().file(x).file_split(x, Period::Day).max_log_level(LevelFilter::Info).build()?.init().expect("log error");
	} else {
		log!(Level::Info,"Not Logs");
	}
	return Ok(());
}