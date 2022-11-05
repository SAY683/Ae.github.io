use actix_web::main;
use anyhow::Result;
use async_trait::async_trait;

#[main]
async fn main() -> Result<()> {
	return Ok(());
}

pub mod qic_server {
	use super::*;
	use s2n_quic::client::Connect;
	use s2n_quic::{Client, Server};
	use std::future::IntoFuture;
	use std::net::SocketAddr;
	use std::path::Path;
	
	///#HDFS
	#[derive(Debug)]
	pub struct HdfsQuick {
		//服务
		pub client: Client,
		pub server: Server,
	}
	
	#[async_trait]
	pub trait Quick<G = Option<HdfsQuick>> {
		///#服务
		fn server(cert: &str, key: &str, host: &str) -> Result<Server> {
			return Ok(Server::builder()
				.with_tls((Path::new(cert), Path::new(key)))?
				.with_io(host)?
				.start()
				.unwrap());
		}
		///#链接
		fn client(cert: &str) -> Result<Client> {
			return Ok(Client::builder()
				.with_tls(Path::new(cert))?
				.with_io("0.0.0.0:0")?
				.start()
				.unwrap());
		}
		///#Connect::new(e).with_server_name(r);包装
		fn client_connect(e: SocketAddr, r: &str) -> Connect {
			return Connect::new(e).with_server_name(r);
		}
		fn the_thread_of_execution(&self) -> Result<G>;
	}
	
	#[async_trait]
	pub trait QuiContinue: Quick + Sized + IntoFuture {
		type Data;
		type Perform;
		async fn server_wait(&mut self, _: Self::Data) -> Result<Self::Perform>;
		async fn client_wait(&mut self, _: &str, _: Self::Data) -> Result<Self::Perform>;
	}
}

///#密钥
#[derive(Clone)]
pub struct SAR {
	pub cert: String,
	pub key: String,
}

///#HTTP通信 可能的链接方式
pub mod http_server {
	///#http接口
	pub struct HTTPHickey<'life> {
		pub home: &'life str,
	}
	
	impl<'life> Default for HTTPHickey<'life> {
		fn default() -> Self {
			return HTTPHickey {
				home: "127.0.0.1:8080/",
			};
		}
	}
}
