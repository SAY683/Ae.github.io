/*
C++/Java|Python(lib接口)
特殊库(lib)接口
 */
pub mod living_example {
    use jni::objects::*;
    use jni::JNIEnv;
    use std::any::Any;

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
