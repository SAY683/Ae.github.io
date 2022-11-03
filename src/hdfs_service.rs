use std::net::SocketAddr;
use std::path::{PathBuf};
use std::sync::Arc;
use crate::{Master, Slave, SlimeNode, LOCAL_IP, MASTER_MODEL, MODEL, THE_NODE_MODEL};
use std::sync::atomic::Ordering;
use parking_lot::RwLock;
use s2n_quic::{Server, Client};
use tokio::net::{TcpStream, UdpSocket};
use ::HdfsService::SAR;
use MysqlOperating::MysqlHdfsDatabaseDriver;
use RedisOperating::RedisHdfsDatabaseDriver;
use FileOperations::condition::system_environment::SlimeEnvironment;

///#Hdfs
pub struct HdfsManager {
	pub master: Master,
	pub node: Slave,
	pub server: HdfsService,
}

///#[hdfs_service::HdfsManager]实现
pub mod hdfs_manager {
	use super::*;
	
	impl SlimeNode for HdfsManager {
		///#数据返回
		fn new() -> anyhow::Result<Self> {
			return Ok(HdfsManager {
				master: (if MODEL == true {
					Master::new()?
				} else {
					Master::default()
				}),
				node: (if MODEL == true {
					Slave::new()?
				} else {
					Slave::default()
				}),
				server: HdfsService::None,
			});
		}
		type Data = ();
		///#处理节点数据
		fn handle(&self) -> anyhow::Result<Self::Data> {
			if !self.node.slave.is_empty() {
				//有节点
				THE_NODE_MODEL.store(true, Ordering::Release);
			}
			if &self.master.local.ip().to_string() != LOCAL_IP.as_ref().unwrap() {
				//不是master节点
				MASTER_MODEL.store(false, Ordering::Release);
			}
			
			return Ok(());
		}
	}
	
	impl AsRef<HdfsService> for HdfsManager {
		fn as_ref(&self) -> &HdfsService {
			return &self.server;
		}
	}
	
	impl AsRef<Master> for HdfsManager {
		fn as_ref(&self) -> &Master {
			return &self.master;
		}
	}
	
	impl AsRef<Slave> for HdfsManager {
		fn as_ref(&self) -> &Slave {
			return &self.node;
		}
	}
	
	impl From<HdfsService> for HdfsManager {
		fn from(server: HdfsService) -> Self {
			let HdfsManager {
				master, node, server: _
			} = HdfsManager::new().unwrap();
			return HdfsManager {
				master,
				node,
				server,
			};
		}
	}
	
	impl MysqlHdfsDatabaseDriver for HdfsManager {}
	
	impl RedisHdfsDatabaseDriver for HdfsManager {}
}

///#链接通信 默认TCP
#[derive(Clone, Debug)]
pub enum HdfsService {
	///#服务TCP
	ServiceTcp {
		host: SocketAddr,
		server: Arc<RwLock<TcpStream>>,
	},
	///#服务UDP
	ServiceUDP {
		host: SocketAddr,
		server: Arc<RwLock<UdpSocket>>,
	},
	//没有->没有->没有->通过
	None,
	///#服务
	ServiceQUIC {
		host: SocketAddr,
		server: Option<Arc<RwLock<Server>>>,
		client: Option<Arc<RwLock<Client>>>,
	},
}

///#[hdfs_service::HdfsService]
pub mod hdfs_service {
	use super::*;
	
	impl From<(SocketAddr, Server)> for HdfsService {
		fn from(value: (SocketAddr, Server)) -> Self {
			return HdfsService::ServiceQUIC {
				host: value.0,
				server: Some(Arc::new(RwLock::new(value.1))),
				client: None,
			};
		}
	}
	
	impl From<(SocketAddr, Client)> for HdfsService {
		fn from(value: (SocketAddr, Client)) -> Self {
			return HdfsService::ServiceQUIC {
				host: value.0,
				server: None,
				client: Some(Arc::new(RwLock::new(value.1))),
			};
		}
	}
	
	impl From<(SocketAddr, TcpStream)> for HdfsService {
		fn from(value: (SocketAddr, TcpStream)) -> Self {
			return HdfsService::ServiceTcp { host: value.0, server: Arc::new(RwLock::new(value.1)) };
		}
	}
	
	impl From<(SocketAddr, UdpSocket)> for HdfsService {
		fn from(value: (SocketAddr, UdpSocket)) -> Self {
			return HdfsService::ServiceUDP { host: value.0, server: Arc::new(RwLock::new(value.1)) };
		}
	}
}

impl SlimeNode for SAR {
	fn new() -> anyhow::Result<Self> {
		return Ok(SAR { cert: Master::local_path("CERT")?, key: Master::local_path("KEY")? });
	}
	type Data = (PathBuf, PathBuf);
	fn handle(&self) -> anyhow::Result<Self::Data> {
		return Ok((PathBuf::from(self.cert.as_str()), PathBuf::from(self.key.as_str())));
	}
}
