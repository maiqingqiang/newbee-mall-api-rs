use crate::bootstrap::database::PooledConn;
use crate::models::pagination::{Paginate, Paginator};
use crate::models::schema::tb_newbee_mall_carousel::dsl;
use crate::models::{DELETED, NOT_DELETE};
use chrono::{Local, NaiveDateTime};
use diesel::dsl::IntoBoxed;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use serde::Serialize;
use crate::debug_sql;

#[derive(Debug, Queryable, Serialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_carousel)]
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

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_carousel)]
pub struct NewCarousel {
    pub carousel_url: String,
    pub redirect_url: String,
    pub carousel_rank: i32,
    pub create_time: NaiveDateTime,
    pub create_user: i32,
}

sql_function!(fn last_insert_id() -> Integer);

impl Carousel {
    fn filter() -> IntoBoxed<'static, dsl::tb_newbee_mall_carousel, Mysql> {
        let query = dsl::tb_newbee_mall_carousel.into_boxed();
        query
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .order(dsl::carousel_rank.desc())
    }

    pub fn find(conn: &mut PooledConn, carousel_id: i32) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_carousel
            .find(carousel_id)
            .filter(dsl::is_deleted.eq(NOT_DELETE));

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn get_by_limit(conn: &mut PooledConn, limit: i64) -> QueryResult<Vec<Carousel>> {
        let query = Self::filter().limit(limit);

        debug_sql!(&query);

        query.load::<Self>(conn)
    }

    pub fn list(
        conn: &mut PooledConn,
        page_number: Option<i64>,
        page_size: Option<i64>,
    ) -> QueryResult<Paginator<Self>> {
        Paginate::new(Self::filter, page_number)
            .per_page(page_size)
            .load_with_paginator(conn)
    }

    pub fn delete(conn: &mut PooledConn, carousel_ids: Vec<i32>) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_carousel.filter(dsl::carousel_id.eq_any(carousel_ids)))
            .set((
                dsl::is_deleted.eq(DELETED),
                dsl::update_time.eq(Local::now().naive_local()),
            ));

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn create(conn: &mut PooledConn, carousel: NewCarousel) -> QueryResult<Self> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_carousel)
            .values(&carousel);

        debug_sql!(&query);

        query.execute(conn)?;

        let query = dsl::tb_newbee_mall_carousel
            .find(last_insert_id());

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn update(conn: &mut PooledConn, carousel: Self) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_carousel.find(carousel.carousel_id))
            .set(carousel);

        debug_sql!(&query);

        query.execute(conn)
    }
}
