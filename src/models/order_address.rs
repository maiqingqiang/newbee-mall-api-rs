use diesel::prelude::*;
use crate::bootstrap::database::PooledConn;
use crate::models::schema;
use crate::models::schema::tb_newbee_mall_order_address::dsl;

#[derive(Debug, Queryable, AsChangeset, Insertable)]
#[diesel(table_name = schema::tb_newbee_mall_order_address)]
pub struct OrderAddress {
    pub order_id: i64,
    pub user_name: String,
    pub user_phone: String,
    pub province_name: String,
    pub city_name: String,
    pub region_name: String,
    pub detail_address: String,
}

impl OrderAddress {
    pub fn create(conn: &mut PooledConn, order_address: Self) -> QueryResult<usize> {
        diesel::insert_into(dsl::tb_newbee_mall_order_address)
            .values(&order_address)
            .execute(conn)
    }

    pub fn find(conn: &mut PooledConn, order_id: i64) -> QueryResult<Self> {
        dsl::tb_newbee_mall_order_address
            .find(order_id)
            .first(conn)
    }

    pub fn delete(conn: &mut PooledConn, order_id: i64) -> QueryResult<usize> {
        diesel::delete(dsl::tb_newbee_mall_order_address)
            .filter(dsl::order_id.eq(order_id))
            .execute(conn)
    }

    pub fn update(conn: &mut PooledConn, order_address: Self) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_order_address.find(order_address.order_id))
            .set(&order_address)
            .execute(conn)
    }
}