use crate::bootstrap::database::PooledConn;
use crate::debug_sql;
use crate::models::schema;
use crate::models::schema::tb_newbee_mall_order_item::dsl;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = schema::tb_newbee_mall_order_item)]
pub struct OrderItem {
    pub order_item_id: i64,
    pub order_id: i64,
    pub goods_id: i64,
    pub goods_name: String,
    pub goods_cover_img: String,
    pub selling_price: i32,
    pub goods_count: i32,
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::tb_newbee_mall_order_item)]
pub struct NewOrderItem {
    pub order_id: i64,
    pub goods_id: i64,
    pub goods_name: String,
    pub goods_cover_img: String,
    pub selling_price: i32,
    pub goods_count: i32,
}

impl OrderItem {
    pub fn create(conn: &mut PooledConn, order_item: NewOrderItem) -> QueryResult<usize> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_order_item).values(&order_item);

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn create_batch(
        conn: &mut PooledConn,
        order_items: Vec<NewOrderItem>,
    ) -> QueryResult<usize> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_order_item).values(order_items);

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn find(conn: &mut PooledConn, order_item_id: i64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_order_item.find(order_item_id);

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn get_by_order_id(conn: &mut PooledConn, order_id: i64) -> QueryResult<Vec<Self>> {
        Self::get_by_order_ids(conn, vec![order_id])
    }

    pub fn get_by_order_ids(conn: &mut PooledConn, order_ids: Vec<i64>) -> QueryResult<Vec<Self>> {
        let query = dsl::tb_newbee_mall_order_item.filter(dsl::order_id.eq_any(order_ids));

        debug_sql!(&query);

        query.load(conn)
    }

    pub fn delete(conn: &mut PooledConn, order_item_id: i64) -> QueryResult<usize> {
        let query = diesel::delete(dsl::tb_newbee_mall_order_item)
            .filter(dsl::order_item_id.eq(order_item_id));

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn update(conn: &mut PooledConn, order_item: Self) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_order_item.find(order_item.order_item_id))
            .set(&order_item);

        debug_sql!(&query);

        query.execute(conn)
    }
}
