use crate::{Master, Slave, SlimeNode, MASTER_MODEL, THE_NODE_MODEL};
use ::HdfsService::qic_server::{HdfsQuick, QuiContinue, Quick};
use ::HdfsService::SAR;
use anyhow::Result;
use parking_lot::RwLock;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::net::{TcpStream, UdpSocket};
use FileOperations::condition::system_environment::SlimeEnvironment;
use MysqlOperating::MysqlHdfsDatabaseDriver;
use RedisOperating::RedisHdfsDatabaseDriver;

//#Hdfs
pub struct HdfsManager {
	pub master: Master,
	pub node: Slave,
	pub server: HdfsService,
}

///#[hdfs_service::HdfsManager]实现
pub mod hdfs_manager {
	use super::*;
	use crate::{LOCAL_IP, SETTINGS};
	
	impl SlimeNode for HdfsManager {
		///#数据返回
		fn new() -> Result<Self> {
			return Ok(HdfsManager {
				master: (if SETTINGS.get().unwrap().default {
					Master::new()?
				} else {
					Master::default()
				}),
				node: (if SETTINGS.get().unwrap().default {
					Slave::new()?
				} else {
					Slave::default()
				}),
				server: HdfsService::None,
			});
		}
		type Data = ();
		///#处理节点数据
		fn handle(&self) -> Result<Self::Data> {
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
				master,
				node,
				server: _,
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
#[derive(Clone)]
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
		//服务必要内容
		key: SAR,
		host: SocketAddr,
	},
	///#默认数据
	Default {
		//服务必要内容
		key: SAR,
		host: SocketAddr,
	},
}

///#[hdfs_service::HdfsService]
pub mod hdfs_service {
	use super::*;
	use async_trait::async_trait;
	use s2n_quic::Connection;
	use std::future::{ready, IntoFuture, Ready};
	use s2n_quic::client::{Connect};
	use crate::{MASTER, SETTINGS, TEST_MASTER};
	
	///#默认QUI
	impl Default for HdfsService {
		fn default() -> Self {
			return HdfsService::Default {
				key: SAR::new().unwrap(),
				host: {
					if SETTINGS.get().unwrap().default {
						TEST_MASTER.get().unwrap().local
					} else {
						MASTER.get().unwrap().local
					}
				},
			};
		}
	}
	
	impl IntoFuture for HdfsService {
		type Output = Self;
		type IntoFuture = Ready<Self::Output>;
		fn into_future(self) -> Self::IntoFuture {
			return ready(self);
		}
	}
	
	#[async_trait]
	impl QuiContinue for HdfsService {
		type Server = Option<Connection>;
		///type Server = Option<Connection>;
		///async fn server_wait(&mut self) -> Result<Self::Server>
		async fn server_wait(&mut self) -> Result<Self::Server> {
			return Ok(
				if let Some(mut x) = HdfsService::the_thread_of_execution(self)? {
					x.server.accept().await
				} else {
					None
				},
			);
		}
		
		type Client = Option<Connection>;
		///type Client = Option<Connection>;
		///async fn client_wait(self) -> Result<Self::Client>
		///idea的rust插件兼容性问题否则可以
		///async fn client_wait(&mut self, host_name: &str) -> Result<Self::Client> {
		///			return Ok(if let Some(x) = HdfsService::the_thread_of_execution(self)? &&let HdfsService::ServiceQUIC { key: _, host } = self{
		///				let mut x = x.client.connect(Connect::new(host.to_string().as_str().parse::<SocketAddr>()?).with_server_name(host_name)).await?;
		///			    x.keep_alive(true)?;
		///				Some(x)
		///			} else {
		///				None
		///			});
		///		}
		async fn client_wait(&mut self, host_name: &str) -> Result<Self::Client> {
			return Ok(if let Some(x) = HdfsService::the_thread_of_execution(self)? {
				if let HdfsService::ServiceQUIC { key: _, host } = self {
					let mut x = x.client.connect(Connect::new(host.to_string().as_str().parse::<SocketAddr>()?).with_server_name(host_name)).await?;
					x.keep_alive(true)?;
					Some(x)
				} else {
					None
				}
			} else {
				None
			});
		}
	}
	
	impl Quick for HdfsService {
		///#返回[HdfsQuick] default支持
		fn the_thread_of_execution(&self) -> Result<Option<HdfsQuick>> {
			return Ok(if let Some((key, host)) = if let HdfsService::ServiceQUIC { key, host } = self {
				Some((key, host))
			} else {
				'block: {
					if let HdfsService::Default { key, host } = self {
						break 'block
							Some((key, host));
					}
					None
				}
			} {
				Some(HdfsQuick {
					client: HdfsService::client(key.cert.as_str(), host.to_string().as_str())?,
					server: HdfsService::server(
						key.cert.as_str(),
						key.key.as_str(),
						host.to_string().as_str(),
					)?,
				})
			} else {
				None
			});
		}
	}
	
	impl From<(SAR, SocketAddr)> for HdfsService {
		///#添加
		fn from(value: (SAR, SocketAddr)) -> Self {
			return HdfsService::ServiceQUIC {
				key: value.0,
				host: value.1,
			};
		}
	}
	
	impl From<(SocketAddr, TcpStream)> for HdfsService {
		fn from(value: (SocketAddr, TcpStream)) -> Self {
			return HdfsService::ServiceTcp {
				host: value.0,
				server: Arc::new(RwLock::new(value.1)),
			};
		}
	}
	
	impl From<(SocketAddr, UdpSocket)> for HdfsService {
		fn from(value: (SocketAddr, UdpSocket)) -> Self {
			return HdfsService::ServiceUDP {
				host: value.0,
				server: Arc::new(RwLock::new(value.1)),
			};
		}
	}
}

impl SlimeNode for SAR {
	fn new() -> Result<Self> {
		return Ok(SAR {
			cert: Master::local_path("CERT")?,
			key: Master::local_path("KEY")?,
		});
	}
	type Data = (PathBuf, PathBuf);
	fn handle(&self) -> Result<Self::Data> {
		return Ok((
			PathBuf::from(self.cert.as_str()),
			PathBuf::from(self.key.as_str()),
		));
	}
}
