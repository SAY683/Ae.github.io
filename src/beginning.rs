use crate::{
    Master, Result, Slave, SlimeMysql, SlimeNode, SlimeRedis, MASTER, MODEL, MYSQL, MYSQL_VERSION,
    REDIS, REDIS_DIR, REDIS_DIR_INIT, REDIS_VERSION, SLAVE, TEST_MASTER, TEST_MYSQL, TEST_REDIS,
    TEST_SLAVE, UNIVERSAL_GLOBAL,
};
use log::{log, Level};
use r2d2_redis::RedisConnectionManager;
use RedisOperating::RedisServer;

///初始
pub async fn beginning(e: bool) -> Result<()> {
    if e {
        log!(Level::Info, "Testing:<{}>MODE", e);
        testing();
        server_setting(e)
            .await
            .unwrap_or_else(|e| log!(Level::Debug, "Data Is Error[{}]", e));
    } else {
        log!(Level::Info, "Execute:<{}>MODE", !e);
        data_ing()?;
        server_setting(e)
            .await
            .unwrap_or_else(|e| log!(Level::Debug, "Data Is Error[{}]", e));
    }
    return Ok(());
}
///#数据链接初始
async fn server_setting(e: bool) -> Result<()> {
    match ping().await? {
        (x, y) if x == y && x == true => {
            if UNIVERSAL_GLOBAL == true {
                let x = if e {
                    RedisConnectionManager::new(TEST_REDIS.get().unwrap().handle()?)?
                } else {
                    RedisConnectionManager::new(REDIS.get().unwrap().handle()?)?
                };
                REDIS_DIR.get_or_init(|| x);
            };
            let x = if e {
                SlimeRedis::get_redis(&TEST_REDIS.get().unwrap().handle()?)?
            } else {
                SlimeRedis::get_redis(&REDIS.get().unwrap().handle()?)?
            };
            REDIS_DIR_INIT.get_or_init(|| x);
        }
        _ => {
            panic!("Basic configuration error")
        }
    };
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
