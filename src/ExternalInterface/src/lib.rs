/*
C++/Java|Python(lib接口)
特殊库(lib)接口
 */
///#转换特宏
#[macro_use]
mod database_strap;

pub mod living_example {
	use jni::objects::*;
	use jni::JNIEnv;
	use std::any::Any;
	
	///#Jvm必须问题配置
	#[cfg(linx)]
	pub fn java_jmv() -> Result<Jvm> {
		use j4rs::{Jvm, JvmBuilder};
		use anyhow::Result;
		return Ok(JvmBuilder::new().build()?);
	}
	
	///#TODO:通用无解析接口
	#[warn(improper_ctypes_definitions)]
	#[no_mangle]
	pub extern "C" fn Java_Star_taste(_: *const Box<dyn Any>, _: *const Box<dyn Any>) {
		println!("Yes This Rust");
	}
	
	///#TODO:Java_<类完整路径>_<方法名>
	#[no_mangle]
	#[allow(non_snake_case)]
	pub extern "system" fn Java_HelloWorld_hello(env: JNIEnv, _class: JClass, input: JString) {
		let input: String = env
			.get_string(input)
			.expect("Couldn't get java string!")
			.into();
		env.new_string(format!("Hello, {}!", input))
			.expect("Couldn't create java string!");
	}
}