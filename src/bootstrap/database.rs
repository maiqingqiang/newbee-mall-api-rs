use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};

pub type DatabasePool = Pool<ConnectionManager<MysqlConnection>>;
pub type PooledConn = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn connection() -> DatabasePool {
    let manager = ConnectionManager::<MysqlConnection>::new(&crate::config::DATABASE.url);

    Pool::builder()
        .min_idle(Some(5))
        .max_size(15)
        .build(manager)
        .expect("创建连接池失败")
}
