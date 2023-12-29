use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

use crate::bootstrap::database::PooledConn;
use crate::debug_sql;
use crate::models::schema;
use crate::models::schema::tb_newbee_mall_user_address::dsl;
use crate::models::{DELETED, NOT_DELETE};

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = schema::tb_newbee_mall_user_address)]
pub struct UserAddress {
    pub address_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub user_phone: String,
    pub default_flag: i8,
    pub province_name: String,
    pub city_name: String,
    pub region_name: String,
    pub detail_address: String,
    pub is_deleted: i8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::tb_newbee_mall_user_address)]
pub struct NewUserAddress {
    pub user_id: i64,
    pub city_name: String,
    pub default_flag: i8,
    pub detail_address: String,
    pub province_name: String,
    pub region_name: String,
    pub user_name: String,
    pub user_phone: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = schema::tb_newbee_mall_user_address)]
pub struct UpdateDefaultFlag {
    pub default_flag: i8,
    pub update_time: NaiveDateTime,
}

impl UserAddress {
    pub const NOT_DEFAULT: i8 = 0;
    pub const DEFAULTED: i8 = 1;

    pub fn list(conn: &mut PooledConn, user_id: i64) -> QueryResult<Vec<Self>> {
        let query = dsl::tb_newbee_mall_user_address
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .limit(20)
            .order(dsl::address_id.desc());

        debug_sql!(&query);

        query.load::<Self>(conn)
    }

    pub fn create(conn: &mut PooledConn, user_address: NewUserAddress) -> QueryResult<usize> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_user_address).values(&user_address);

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn update(conn: &mut PooledConn, user_address: Self) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_user_address.find(user_address.address_id))
            .set(user_address);

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn update_default_flag(
        conn: &mut PooledConn,
        user_id: i64,
        default_flag: i8,
    ) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_user_address)
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .filter(dsl::default_flag.eq(Self::DEFAULTED))
            .set(UpdateDefaultFlag {
                default_flag,
                update_time: Local::now().naive_local(),
            });

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn find(conn: &mut PooledConn, address_id: i64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_user_address.find(address_id);

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn find_default(conn: &mut PooledConn, user_id: i64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_user_address
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .filter(dsl::default_flag.eq(Self::DEFAULTED));

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn delete_by_soft(conn: &mut PooledConn, address_id: i64) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_user_address.find(address_id))
            .set(dsl::is_deleted.eq(DELETED));

        debug_sql!(&query);

        query.execute(conn)
    }
}
