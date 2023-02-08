use crate::bootstrap::database::PooledConn;
use crate::models::pagination::Paginator;
use crate::models::schema::tb_newbee_mall_order::{dsl, BoxedQuery};
use crate::models::{schema, DELETED};
use chrono::{Local, NaiveDateTime};
use diesel::mysql::Mysql;
use diesel::prelude::*;

use super::pagination::Paginate;

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = schema::tb_newbee_mall_order)]
pub struct Order {
    pub order_id: i64,
    pub order_no: String,
    pub user_id: i64,
    pub total_price: i32,
    pub pay_status: i8,
    pub pay_type: i8,
    pub pay_time: Option<NaiveDateTime>,
    pub order_status: i8,
    pub extra_info: String,
    pub is_deleted: i8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::tb_newbee_mall_order)]
pub struct NewOrder {
    pub order_no: String,
    pub user_id: i64,
    pub total_price: i32,
    pub extra_info: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = schema::tb_newbee_mall_order)]
struct UpdateOrderStatus {
    pub order_status: i8,
    pub update_time: NaiveDateTime,
}

pub struct Filter {
    pub status: Option<i8>,
    pub page_number: Option<i64>,
}

impl Order {
    fn filter(filter: &Filter) -> BoxedQuery<Mysql> {
        let mut query = dsl::tb_newbee_mall_order.into_boxed();

        if let Some(status) = filter.status {
            query = query.filter(dsl::order_status.eq(status));
        }

        query
    }

    pub fn count(conn: &mut PooledConn, filter: &Filter) -> QueryResult<i64> {
        Self::filter(filter).count().get_result(conn)
    }

    pub fn get_with_paginator(
        conn: &mut PooledConn,
        filter: &Filter,
    ) -> QueryResult<Paginator<Order>> {
        Paginate::new(|| Self::filter(filter), filter.page_number).load_with_paginator(conn)
    }

    pub fn delete_by_soft(conn: &mut PooledConn, order_id: i64) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_order.find(order_id))
            .set(dsl::is_deleted.eq(DELETED))
            .execute(conn)
    }

    pub fn find(conn: &mut PooledConn, order_id: i64) -> QueryResult<Self> {
        dsl::tb_newbee_mall_order.find(order_id).first(conn)
    }

    pub fn find_by_order_no(conn: &mut PooledConn, order_no: String) -> QueryResult<Self> {
        dsl::tb_newbee_mall_order
            .filter(dsl::order_no.eq(order_no))
            .first(conn)
    }

    pub fn list_by_order_ids(conn: &mut PooledConn, order_ids: Vec<i64>) -> QueryResult<Vec<Self>> {
        dsl::tb_newbee_mall_order
            .filter(dsl::order_id.eq_any(order_ids))
            .load::<Self>(conn)
    }

    pub fn create(conn: &mut PooledConn, order: NewOrder) -> QueryResult<Self> {
        diesel::insert_into(dsl::tb_newbee_mall_order)
            .values(&order)
            .execute(conn)?;

        dsl::tb_newbee_mall_order
            .find(super::functions::last_insert_id())
            .first(conn)
    }

    pub fn update_order_status(
        conn: &mut PooledConn,
        order_ids: Vec<i64>,
        order_status: i8,
    ) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_order)
            .filter(dsl::order_id.eq_any(order_ids))
            .set(&UpdateOrderStatus {
                order_status,
                update_time: Local::now().naive_local(),
            })
            .execute(conn)
    }

    pub fn update(conn: &mut PooledConn, order: Self) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_order)
            .filter(dsl::order_id.eq(order.order_id))
            .set(order)
            .execute(conn)
    }
}
