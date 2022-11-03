use serde::{Serialize, Deserialize};

///#应用设置
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationSettings {
	#[serde(rename = "default", default)]
	pub default: bool,
}