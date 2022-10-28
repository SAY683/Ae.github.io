pub mod mysql {
    use crate::node_data::Node;
    use crate::{Master, SlimeNode, MODEL, MYSQL, MYSQL_DIR_INIT, TEST_MYSQL};
    use anyhow::Result;
    use async_trait::async_trait;
    use rbatis::Rbatis;
    use serde::{Deserialize, Serialize};
    use MysqlOperating::{AeExam, MysqlOrm, MysqlServer};
    use PropertyMacro::MysqlServer;

    ///#存储位置 master不会直接存储文件
    #[derive(Debug, Serialize, Deserialize, MysqlServer)]
    pub struct StorageLocation {
        pub node: Vec<Node>,
    }

    impl StorageLocation {
        ///#MysqlOrm
        pub async fn get_mysql<RX: Sized + MysqlOrm>() -> Result<Rbatis> {
            if MODEL == true {
                Ok(RX::orm_get(&TEST_MYSQL.get().unwrap().handle()?).await?)
            } else {
                Ok(RX::orm_get(&MYSQL.get().unwrap().handle()?).await?)
            }
        }
    }
    #[async_trait]
    impl MysqlOrm for Master {
        type Object = Vec<AeExam>;
        async fn orm_select() -> anyhow::Result<Self::Object> {
            let mut e: Rbatis = StorageLocation::get_mysql::<Master>().await?;
            return Ok(AeExam::select_all(&mut e).await?.into_iter().collect());
        }
        type Data = Vec<String>;
        async fn orm_database_node(&self) -> anyhow::Result<Self::Data> {
            let mut x = MYSQL_DIR_INIT.as_ref().unwrap();
            return Ok(AeExam::select_all(&mut x)
                .await?
                .into_iter()
                .filter(|x| x.location.is_some())
                .map(|x| -> String {
                    serde_json::to_string(&x.location.unwrap().to_string()).unwrap()
                })
                .collect());
        }
        async fn orm_insert(e: Self::Object) -> Result<Self::Object> {
            let mut x = MYSQL_DIR_INIT.as_ref().unwrap();
            for i in e.iter() {
                AeExam::insert(&mut x, i).await?;
            }
            return Ok(e);
        }
    }
}
