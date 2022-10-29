pub mod mysql {
    use crate::node_data::Node;
    use crate::{Master, SlimeNode, MODEL, MYSQL, MYSQL_DIR_INIT, TEST_MYSQL};
    use anyhow::Result;
    use async_trait::async_trait;
    use rbatis::Rbatis;
    use serde::{Deserialize, Serialize};
    use MysqlOperating::{AeExam, MysqlOrm, MysqlServer};
    use PropertyMacro::MysqlServer;
    use RedisOperating::RedisServerPoll;

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
        ///#async fn orm_insert(e: Self::Object) -> Result<Self::Object>
        ///#type Object = Vec<AeExam>;
        ///[Master::get_redis_set]
        async fn orm_insert(e: Self::Object) -> Result<()> {
            let mut x = MYSQL_DIR_INIT.as_ref().unwrap();
            let mut r = Vec::new();
            for i in e.into_iter() {
                AeExam::insert(&mut x, &i).await?;
                r.push((i.name.to_string(), i.id.unwrap().as_str().to_string()));
            }
            Master::get_redis_set(&r).await?;
            return Ok(());
        }
        type DataTable = AeExam;
        ///#name/id r=name
        ///#AeExam ID = None
        async fn orm_update(e: Self::DataTable, r: String) -> Result<Self::DataTable> {
            let v = AeExam {
                id: Some(Master::get_redis_get(&r).await?.unwrap_or_else(|| {
                    eprintln!("Redis Is None");
                    String::new()
                })),
                ..e
            };
            //查询
            AeExam::update_by_column(&mut MYSQL_DIR_INIT.as_ref().unwrap(), &v, "id").await?;
            return Ok(v);
        }
        ///#删除where r
        async fn orm_remove(r: String) -> Result<()> {
            AeExam::delete_by_column(
                &mut MYSQL_DIR_INIT.as_ref().unwrap(),
                "id",
                Master::get_redis_get(&r)
                    .await?
                    .unwrap_or_else(|| {
                        eprintln!("Redis Is None");
                        String::new()
                    })
                    .as_str(),
            )
            .await?;
            return Ok(Master::get_redis_remove(&r).await?);
        }
    }
}
pub mod redis {
    use crate::node_data::Master;
    use crate::REDIS_DIR_INIT;
    use async_trait::async_trait;
    use deadpool_redis::redis::cmd;
    use RedisOperating::RedisServerPoll;
    #[async_trait]
    impl RedisServerPoll for Master {
        type Data = Vec<String>;
        ///#async fn get_redis_set(e: &Vec<(String, String)>) -> anyhow::Result<Self::Data>
        async fn get_redis_set(e: &Vec<(String, String)>) -> anyhow::Result<Self::Data> {
            let mut z = REDIS_DIR_INIT
                .as_ref()
                .unwrap()
                .get_async_connection()
                .await?;
            for (x, y) in e.iter() {
                cmd("SET").arg(x).arg(y).query_async(&mut z).await?;
            }
            return Ok(vec![]);
        }
        ///#async fn get_redis_get(e: &String) -> anyhow::Result<Option<String>>
        async fn get_redis_get(e: &String) -> anyhow::Result<Option<String>> {
            return Ok(cmd("GET").arg(e).query::<Option<String>>(
                &mut REDIS_DIR_INIT.as_ref().unwrap().get_connection()?,
            )?);
        }
        ///#async fn get_redis_remove(e: &String) -> anyhow::Result<()>
        async fn get_redis_remove(e: &String) -> anyhow::Result<()> {
            return Ok(cmd("DEL")
                .arg(e)
                .query_async(
                    &mut REDIS_DIR_INIT
                        .as_ref()
                        .unwrap()
                        .get_async_connection()
                        .await?,
                )
                .await?);
        }
    }
}
