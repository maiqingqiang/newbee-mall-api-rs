use crate::bootstrap::database::PooledConn;
use crate::models::schema::tb_newbee_mall_carousel::dsl::tb_newbee_mall_carousel;
use crate::models::schema::tb_newbee_mall_carousel::{carousel_rank, is_deleted};
use crate::models::NOT_DELETE;
use chrono::NaiveDateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Carousel {
    pub carousel_id: i32,
    pub carousel_url: String,
    pub redirect_url: String,
    pub carousel_rank: i32,
    pub is_deleted: i8,
    pub create_time: NaiveDateTime,
    pub create_user: i32,
    pub update_time: NaiveDateTime,
    pub update_user: i32,
}

impl Carousel {
    pub fn list(conn: &mut PooledConn, limit: i64) -> QueryResult<Vec<Carousel>> {
        tb_newbee_mall_carousel::table()
            .filter(is_deleted.eq(NOT_DELETE))
            .order(carousel_rank.desc())
            .limit(limit)
            .load::<Self>(conn)
    }
}
