/*
环境数据
 */
///#系统数据
pub mod system_files {
    ///#系统数据
    pub struct SystemData {
        ///#真实名称
        pub rename: String,
        ///#用户名称
        pub username: String,
        ///#用户语言选择
        pub lang: Vec<String>,
        ///#设备名称
        pub devices: String,
        ///#主机名称
        pub hosts: String,
        ///#平台
        pub platform: String,
        ///#系统名称
        pub system_name: String,
        ///#桌面环境
        pub desktop_environment: String,
    }

    impl Default for SystemData {
        fn default() -> Self {
            return SystemData {
                rename: whoami::realname(),
                username: whoami::username(),
                lang: whoami::lang().collect::<Vec<String>>(),
                devices: whoami::devicename(),
                hosts: whoami::hostname(),
                platform: whoami::platform().to_string(),
                system_name: whoami::distro(),
                desktop_environment: whoami::desktop_env().to_string(),
            };
        }
    }
}
///#系统环境
pub mod system_environment {
    use crate::condition::system_files::SystemData;
    use anyhow::Result;
    use dotenv::dotenv;
    use std::env::{current_dir, set_var, var};
    use std::path::PathBuf;
    use walkdir::{DirEntry, WalkDir};

    ///#文件操作
    ///#同步线程
    pub trait SlimeEnvironment {
        //#环境变量读取
        fn local_path(e: &str) -> Result<String> {
            dotenv().ok();
            return Ok(var(e)?);
        }
        //#本机数据
        fn local_data() -> SystemData {
            return SystemData::default();
        }
        //#当前路径
        fn environment_variable() -> Result<PathBuf> {
            return Ok(current_dir()?);
        }
        //#添加环境变量
        fn add_variable(k: &str, y: &str) {
            set_var(k, y);
        }
        //#文件查询
        fn file_query(j: &str, i: usize) -> Vec<DirEntry> {
            return WalkDir::new(j)
                .min_depth(1)
                .max_depth(i)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .collect::<Vec<_>>();
        }
    }
}
