use serde::{Serialize, Deserialize};

///#应用设置
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationSettings {
	#[serde(rename = "setting", default)]
	pub default: Vec<Setting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
	pub default: bool,
	pub logs: bool,
}