use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::debug_sql;
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

pub struct Filter {
    pub status: Option<i8>,
    pub page_number: Option<i64>,
}

pub enum OrderStatus {
    PrePay = 0,
    Paid = 1,
    Packaged = 2,
    Express = 3,
    Success = 4,
    ClosedByMalluser = -1,
    ClosedByExpired = -2,
    ClosedByJudge = -3,
}

impl OrderStatus {
    pub(crate) fn from_i8(i: i8) -> result::Result<OrderStatus> {
        match i {
            0 => Ok(Self::PrePay),
            1 => Ok(Self::Paid),
            2 => Ok(Self::Packaged),
            3 => Ok(Self::Express),
            4 => Ok(Self::Success),
            -1 => Ok(Self::ClosedByMalluser),
            -2 => Ok(Self::ClosedByExpired),
            -3 => Ok(Self::ClosedByJudge),
            _ => Err("不支持当前类型".into()),
        }
    }

    pub(crate) fn get_description(&self) -> String {
        match self {
            Self::PrePay => "待支付".to_string(),
            Self::Paid => "已支付".to_string(),
            Self::Packaged => "配货完成".to_string(),
            Self::Express => "出库成功".to_string(),
            Self::Success => "交易成功".to_string(),
            Self::ClosedByMalluser => "手动关闭".to_string(),
            Self::ClosedByExpired => "超时关闭".to_string(),
            Self::ClosedByJudge => "商家关闭".to_string(),
        }
    }
}

impl Order {
    fn filter(filter: &Filter) -> BoxedQuery<Mysql> {
        let mut query = dsl::tb_newbee_mall_order.into_boxed();

        if let Some(status) = filter.status {
            query = query.filter(dsl::order_status.eq(status));
        }

        query
    }

    pub fn get_with_paginator(
        conn: &mut PooledConn,
        filter: &Filter,
    ) -> QueryResult<Paginator<Order>> {
        Paginate::new(|| Self::filter(filter), filter.page_number).load_with_paginator(conn)
    }

    pub fn delete_by_soft(conn: &mut PooledConn, order_id: i64) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_order.find(order_id))
            .set(dsl::is_deleted.eq(DELETED));

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn find(conn: &mut PooledConn, order_id: i64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_order.find(order_id);

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn find_by_order_no(conn: &mut PooledConn, order_no: String) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_order.filter(dsl::order_no.eq(order_no));

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn list_by_order_ids(conn: &mut PooledConn, order_ids: Vec<i64>) -> QueryResult<Vec<Self>> {
        let query = dsl::tb_newbee_mall_order.filter(dsl::order_id.eq_any(order_ids));

        debug_sql!(&query);

        query.load::<Self>(conn)
    }

    pub fn create(conn: &mut PooledConn, order: NewOrder) -> QueryResult<Self> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_order).values(&order);

        debug_sql!(&query);

        query.execute(conn)?;

        let query = dsl::tb_newbee_mall_order.find(super::functions::last_insert_id());

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn update_order_status(
        conn: &mut PooledConn,
        order_ids: Vec<i64>,
        order_status: i8,
    ) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_order)
            .filter(dsl::order_id.eq_any(order_ids))
            .set((
                dsl::order_status.eq(order_status),
                dsl::update_time.eq(Local::now().naive_local()),
            ));

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn update(conn: &mut PooledConn, order: Self) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_order)
            .filter(dsl::order_id.eq(order.order_id))
            .set(order);

        debug_sql!(&query);

        query.execute(conn)
    }
}
