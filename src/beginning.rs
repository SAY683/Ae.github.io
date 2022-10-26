use crate::{
    Master, Result, Slave, SlimeMysql, SlimeNode, SlimeRedis, MASTER, MYSQL, MYSQL_VERSION, REDIS,
    REDIS_VERSION, SLAVE, TEST_MASTER, TEST_MYSQL, TEST_REDIS, TEST_SLAVE,
};
use log::{log, Level};

///初始
pub async fn beginning(e: bool) -> Result<()> {
    if e {
        log!(Level::Info, "Testing:<{}>MODE", e);
        testing();
        match ping().await? {
            (x, y) if x == y && x == true => {}
            _ => {
                panic!("Basic configuration error")
            }
        }
    } else {
        log!(Level::Info, "Execute:<{}>MODE", !e);
        data_ing()?;
        match ping().await? {
            (x, y) if x == y && x == true => {}
            _ => {
                panic!("Basic configuration error")
            }
        }
    }
    return Ok(());
}
///#服务器链接测试 Mysql|Redis|
pub async fn ping() -> Result<(bool, bool)> {
    return Ok((
        *MYSQL_VERSION.as_ref().unwrap_or_else(|_| {
            log!(Level::Debug, "Master_Mysql_Error");
            &false
        }),
        *REDIS_VERSION.as_ref().unwrap_or_else(|_| {
            log!(Level::Debug, "Master_Redis_Error");
            &false
        }),
    ));
}
///#数据初始
fn data_ing() -> Result<()> {
    let x = Master::new()?;
    MASTER.get_or_init(|| x);
    let x = Slave::new()?;
    SLAVE.get_or_init(|| x);
    let x = SlimeMysql::new()?;
    MYSQL.get_or_init(|| x);
    let x = SlimeRedis::new()?;
    REDIS.get_or_init(|| x);
    return Ok(());
}
///#数据测试
fn testing() {
    TEST_MASTER.get_or_init(|| Master::default());
    TEST_SLAVE.get_or_init(|| Slave::default());
    TEST_MYSQL.get_or_init(|| SlimeMysql::default());
    TEST_REDIS.get_or_init(|| SlimeRedis::default());
}
