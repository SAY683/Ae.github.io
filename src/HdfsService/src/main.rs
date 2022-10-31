use std::path::Path;
use actix_web::main;
use anyhow::Result;

#[main]
async fn main() -> Result<()> {
	return Ok(());
}

///#密钥
pub struct SAR {
	pub cert: Path,
	pub key: Path,
}

///#HTTP通信 可能的链接方式
pub mod http_server {}

pub mod qic_server {}