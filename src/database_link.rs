pub mod mysql {
    use crate::node_data::Node;
    use crate::{Master, SlimeNode, MODEL, MYSQL, TEST_MYSQL};
    use async_trait::async_trait;
    use rbatis::Rbatis;
    use serde::{Deserialize, Serialize};
    use MysqlOperating::{AeExam, MysqlOrm};

    ///#存储位置 master不会直接存储文件
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageLocation {
        pub node: Vec<Node>,
    }
    #[async_trait]
    impl MysqlOrm for Master {
        type Data = Vec<String>;
        async fn orm_database_node(&self) -> anyhow::Result<Self::Data> {
            let mut e: Rbatis = if MODEL == true {
                Master::orm_get(&TEST_MYSQL.get().unwrap().handle()?).await?
            } else {
                Master::orm_get(&MYSQL.get().unwrap().handle()?).await?
            };
            let mut u = vec![];
            AeExam::select_all(&mut e).await?.into_iter().for_each(|x| {
                if x.location.is_some() {
                    u.push(serde_json::to_string(&x.location.unwrap().to_string()).unwrap());
                }
            });
            return Ok(u);
        }
    }
}
