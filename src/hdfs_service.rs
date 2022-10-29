use crate::{Master, Slave, SlimeNode, LOCAL_IP, MASTER_MODEL, MODEL, THE_NODE_MODEL};
use std::sync::atomic::Ordering;
use MysqlOperating::MysqlHdfsDatabaseDriver;
use RedisOperating::RedisHdfsDatabaseDriver;

///#Hdfs
pub struct HdfsManager {
    pub master: Master,
    pub node: Slave,
}

impl SlimeNode for HdfsManager {
    ///#数据返回
    fn new() -> anyhow::Result<Self> {
        return Ok(HdfsManager {
            master: (if MODEL == true {
                Master::new()?
            } else {
                Master::default()
            }),
            node: (if MODEL == true {
                Slave::new()?
            } else {
                Slave::default()
            }),
        });
    }
    type Data = HdfsService;
    ///#处理节点数据
    fn handle(&self) -> anyhow::Result<Self::Data> {
        if !self.node.slave.is_empty() {
            //有节点
            THE_NODE_MODEL.store(true, Ordering::Release);
        }
        if &self.master.local.ip().to_string() != LOCAL_IP.as_ref().unwrap() {
            //不是master节点
            MASTER_MODEL.store(false, Ordering::Relaxed);
        }
        return Ok(HdfsService {});
    }
}
///#HdfsService
pub struct HdfsService {}

impl MysqlHdfsDatabaseDriver for HdfsManager {}

impl RedisHdfsDatabaseDriver for HdfsManager {}
