#![feature(
    arbitrary_enum_discriminant,
    type_alias_impl_trait,
    atomic_from_mut,
    inline_const,
    const_mut_refs,
    associated_type_defaults,
    array_zip,
    box_syntax,
    unboxed_closures,
    async_closure
)]
/*
存储HDFS
 */
use std::mem::ManuallyDrop;
use MysqlOperating::MysqlServer;
use RedisOperating::RedisServer;

pub fn main() {}
pub union Service<R: Sized, G: Sized>
where
    R: MysqlServer,
    G: RedisServer,
{
    pub mysql: ManuallyDrop<R>,
    pub redis: ManuallyDrop<G>,
}
