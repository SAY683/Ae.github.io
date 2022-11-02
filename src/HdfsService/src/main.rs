use std::path::{PathBuf};
use actix_web::main;
use anyhow::Result;

#[main]
async fn main() -> Result<()> {
	return Ok(());
}

///#密钥
pub struct SAR {
	pub cert: PathBuf,
	pub key: PathBuf,
}

///#HTTP通信 可能的链接方式
pub mod http_server {
	///#http接口
	pub struct HTTPHickey<'life> {
		pub home: &'life str,
	}
}

pub mod qic_server {}