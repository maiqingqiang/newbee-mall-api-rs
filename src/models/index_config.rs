use crate::bootstrap::database::PooledConn;
use crate::models::schema::tb_newbee_mall_index_config::dsl::tb_newbee_mall_index_config;
use crate::models::schema::tb_newbee_mall_index_config::{config_rank, config_type, is_deleted};
use crate::models::NOT_DELETE;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::Serialize;
use crate::debug_sql;

pub const INDEX_SEARCH_HOTS: i8 = 1;
pub const INDEX_SEARCH_DOWN_HOTS: i8 = 2;
pub const INDEX_GOODS_HOT: i8 = 3;
pub const INDEX_GOODS_NEW: i8 = 4;
pub const INDEX_GOODS_RECOMMOND: i8 = 5;

#[derive(Debug, Queryable, Serialize)]
pub struct IndexConfig {
    pub config_id: i64,
    pub config_name: String,
    pub config_type: i8,
    pub goods_id: i64,
    pub redirect_url: String,
    pub config_rank: i32,
    pub is_deleted: i8,
    pub create_time: NaiveDateTime,
    pub create_user: i32,
    pub update_time: NaiveDateTime,
    pub update_user: Option<i32>,
}

impl IndexConfig {
    pub fn list(conn: &mut PooledConn, t: i8, limit: i64) -> QueryResult<Vec<IndexConfig>> {
        let query = tb_newbee_mall_index_config::table()
            .filter(config_type.eq(t))
            .filter(is_deleted.eq(NOT_DELETE))
            .order(config_rank.desc())
            .limit(limit);

        debug_sql!(&query);

        query.load::<IndexConfig>(conn)
    }
}
