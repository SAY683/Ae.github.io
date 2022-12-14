use anyhow::Result;
use async_trait::async_trait;
use compact_str::CompactString;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use FileOperations::condition::system_environment::SlimeEnvironment;
use FileOperations::local_data;
use FileOperations::local_data::{FileOperation, LocalFileOperations};
use MysqlOperating::MysqlServer;
use PropertyMacro::{MysqlServer, RedisServer, SlimeEnvironment};
use RedisOperating::RedisServer;

///#Master数据
#[derive(Debug, Serialize, Deserialize, SlimeEnvironment, MysqlServer, RedisServer)]
pub struct Master {
	pub local: SocketAddr,
	pub hdfs: PathBuf,
	pub logs: PathBuf,
}

///#节点数据
#[derive(Debug, Serialize, Deserialize, SlimeEnvironment, MysqlServer, RedisServer)]
pub struct Slave {
	//节点
	pub slave: Vec<Node>,
	//slave_hdfs统一配置
	pub hdfs: PathBuf,
	//slave守护节点
	pub guard: Node,
}

///#节点
#[derive(Debug, Serialize, Deserialize, SlimeEnvironment)]
pub struct Node {
	//名称
	pub name: String,
	//host
	pub host: String,
}

pub trait SlimeNode: Sized {
	fn new() -> Result<Self>;
	//产生
	fn target(dir: &str, file: &str) -> Result<Vec<(PathBuf, RwLock<CompactString>)>> {
		return Ok(LocalFileOperations([local_data::FileOperations::Read([(
			CompactString::new(dir),
			vec![CompactString::new(file)],
		)])])
			.run()?);
	}
	type Data;
	//处理
	fn handle(&self) -> Result<Self::Data>;
}

///#Redis
pub mod redis_ulr {
	use crate::{SlimeNode, REDIS_PORT_INIT};
	use serde_json::from_str;
	use RedisOperating::SlimeRedis;
	
	pub struct SlimeRedisServer(pub SlimeRedis);
	
	impl SlimeNode for SlimeRedis {
		fn new() -> anyhow::Result<Self> {
			let x = SlimeRedis::target(REDIS_PORT_INIT[0], REDIS_PORT_INIT[1])?;
			let x = x.get(0).unwrap();
			return Ok(from_str(&*x.1.read().as_str())?);
		}
		type Data = String;
		///#产生
		///#redis://[<username>][:<password>@]<hostname>[:port][/<db>]
		fn handle(&self) -> anyhow::Result<Self::Data> {
			Ok(if self.name.is_some() || self.password.is_some() {
				format!(
					"redis://{}:{}@{}:{}/{}",
					self.name.as_ref().unwrap().as_str(),
					self.password.as_ref().unwrap().as_str(),
					self.ip.as_str(),
					self.port.as_str(),
					self.database.as_str()
				)
			} else {
				format!("redis://{}:{}", self.ip.as_str(), self.port.as_str())
			})
		}
	}
}

///#Mysql[use MysqlOperating::SlimeMysql]
pub mod mysql_config {
	use crate::{SlimeNode, MYSQL_PORT_INIT};
	use serde_json::from_str;
	use MysqlOperating::SlimeMysql;
	
	impl SlimeNode for SlimeMysql {
		fn new() -> anyhow::Result<Self> {
			let x = SlimeMysql::target(MYSQL_PORT_INIT[0], MYSQL_PORT_INIT[1])?;
			let x = x.get(0).unwrap();
			return Ok(from_str(&*x.1.read().as_str())?);
		}
		type Data = String;
		fn handle(&self) -> anyhow::Result<Self::Data> {
			return Ok(format!(
				"mysql://{}:{}@{}/{}",
				self.name.as_str(),
				self.password.as_str(),
				self.host.as_str(),
				self.database
			));
		}
	}
}

///#Slave[create::node_data::Slave]
pub mod slave {
	use super::*;
	use crate::NODE_INIT;
	use hashbrown::{HashMap, HashSet};
	use serde_json::from_str;
	use std::ops::{Deref, DerefMut};
	use std::vec;
	
	impl Default for Slave {
		fn default() -> Self {
			return Slave {
				slave: vec![],
				hdfs: PathBuf::from("./tmp/hdfs"),
				guard: Node {
					name: "".to_string(),
					host: "".to_string(),
				},
			};
		}
	}
	
	impl SlimeNode for Slave {
		fn new() -> Result<Self> {
			let x = Slave::target(NODE_INIT[0], NODE_INIT[1])?;
			let x = x.get(0).unwrap();
			return Ok(from_str(&*x.1.read().as_str())?);
		}
		type Data = RwLock<HashSet<(CompactString, CompactString)>>;
		///type Data = RwLock<HashSet<(CompactString, CompactString)>>;
		fn handle(&self) -> Result<Self::Data> {
			let mut r = HashSet::new();
			self.slave.iter().for_each(|x| {
				r.insert((
					CompactString::new(x.name.as_str()),
					CompactString::new(x.host.as_str()),
				));
			});
			return Ok(RwLock::new(r));
		}
	}
	
	impl IntoIterator for Slave {
		type Item = Node;
		type IntoIter = vec::IntoIter<Self::Item>;
		fn into_iter(self) -> Self::IntoIter {
			return self.slave.into_iter();
		}
	}
	
	impl From<Vec<Node>> for Slave {
		fn from(value: Vec<Node>) -> Self {
			let x = Slave::new().unwrap_or_default();
			return Slave {
				slave: value,
				hdfs: x.hdfs,
				guard: x.guard,
			};
		}
	}
	
	impl From<Node> for Slave {
		fn from(value: Node) -> Self {
			let x = Slave::new().unwrap_or_default();
			return Slave {
				slave: vec![value],
				hdfs: x.hdfs,
				guard: x.guard,
			};
		}
	}
	
	///#T转换&E
	impl AsRef<String> for Node {
		fn as_ref(&self) -> &String {
			return &self.host;
		}
	}
	
	///#mut T转换&mut E
	impl AsMut<String> for Node {
		fn as_mut(&mut self) -> &mut String {
			return &mut self.host;
		}
	}
	
	impl<Rx: ?Sized> AsRef<Rx> for Slave
		where
			<Slave as Deref>::Target: AsRef<Rx>,
	{
		fn as_ref(&self) -> &Rx {
			return self.deref().as_ref();
		}
	}
	
	impl<Rx: ?Sized> AsMut<Rx> for Slave
		where
			<Slave as Deref>::Target: AsMut<Rx>,
	{
		fn as_mut(&mut self) -> &mut Rx {
			return self.deref_mut().as_mut();
		}
	}
	
	impl Deref for Slave {
		type Target = Vec<Node>;
		fn deref(&self) -> &Self::Target {
			return &self.slave;
		}
	}
	
	impl DerefMut for Slave {
		fn deref_mut(&mut self) -> &mut Self::Target {
			return &mut self.slave;
		}
	}
	
	impl Into<HashMap<String, String>> for Slave {
		fn into(self) -> HashMap<String, String> {
			let mut r = HashMap::new();
			self.slave.into_iter().for_each(|x| {
				r.insert(x.name, x.host);
			});
			return r;
		}
	}
}

///#Master[crate::node_data::Master]
pub mod master {
	use super::*;
	use std::ops::{Deref, DerefMut};
	use std::path::PathBuf;
	use quick_xml::de::from_str;
	use FileOperations::program_file_setup::{ApplicationSettings, Setting};
	
	impl SlimeNode for ApplicationSettings {
		fn new() -> Result<Self> {
			let x = Master::target(".", &Master::local_path("AE_XML")?)?;
			let (_, x) = x.get(0).expect("XML Error");
			return Ok(from_str(&*x.read())?);
		}
		type Data = Setting;
		fn handle(&self) -> Result<Self::Data> {
			return Ok(Setting { ..*self.default.get(0).expect("XML Error") });
		}
	}
	
	impl SlimeNode for Master {
		fn new() -> Result<Self> {
			return Ok(Master {
				local: format!(
					"{}:{}",
					Master::local_path("IP")?,
					Master::local_path("PORT")?
				)
					.parse()?,
				hdfs: PathBuf::from(Master::local_path("HDFS")?),
				logs: PathBuf::from(Master::local_path("LOGS")?),
			});
		}
		
		type Data = RwLock<<Master as Deref>::Target>;
		///#type Data = RwLock<HashSet<(CompactString, CompactString)>>;
		fn handle(&self) -> Result<Self::Data> {
			return Ok(RwLock::new(self.local.to_string().parse::<SocketAddr>()?));
		}
	}
	
	impl Default for Master {
		fn default() -> Self {
			return Master {
				local: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8964),
				hdfs: PathBuf::from("./tmp/hdfs"),
				logs: PathBuf::from("./tmp/logs"),
			};
		}
	}
	
	impl Into<String> for Master {
		fn into(self) -> String {
			return self.local.to_string();
		}
	}
	
	impl Deref for Master {
		type Target = SocketAddr;
		fn deref(&self) -> &Self::Target {
			return &self.local;
		}
	}
	
	impl DerefMut for Master {
		fn deref_mut(&mut self) -> &mut Self::Target {
			return &mut self.local;
		}
	}
	
	impl<Rx: Sized> AsRef<Rx> for Master
		where
			<Master as Deref>::Target: AsRef<Rx>,
	{
		fn as_ref(&self) -> &Rx {
			return self.deref().as_ref();
		}
	}
	
	impl<Rx: Sized> AsMut<Rx> for Master
		where
			<Master as Deref>::Target: AsMut<Rx>,
	{
		fn as_mut(&mut self) -> &mut Rx {
			return self.deref_mut().as_mut();
		}
	}
	
	impl From<SocketAddr> for Master {
		///#首先文件否则默认
		fn from(value: SocketAddr) -> Self {
			let x = Master::new().unwrap_or_default();
			return Master {
				local: value,
				hdfs: x.hdfs,
				logs: x.logs,
			};
		}
	}
}
