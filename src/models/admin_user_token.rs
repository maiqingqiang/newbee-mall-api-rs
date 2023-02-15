use crate::bootstrap::database::PooledConn;
use crate::models::schema::tb_newbee_mall_admin_user_token::dsl;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::debug_sql;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_admin_user_token)]
pub struct AdminUserToken {
    pub admin_user_id: i64,
    pub token: String,
    pub update_time: NaiveDateTime,
    pub expire_time: NaiveDateTime,
}

impl AdminUserToken {
    pub fn find(conn: &mut PooledConn, admin_user_id: i64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_admin_user_token
            .find(admin_user_id);

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn find_by_token(conn: &mut PooledConn, token: String) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_admin_user_token
            .filter(dsl::token.eq(token));

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn create(conn: &mut PooledConn, admin_user_token: &AdminUserToken) -> QueryResult<usize> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_admin_user_token)
            .values(admin_user_token);

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn delete(conn: &mut PooledConn, admin_user_id: i64) -> QueryResult<usize> {
        let query = diesel::delete(dsl::tb_newbee_mall_admin_user_token)
            .filter(dsl::admin_user_id.eq(admin_user_id));

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn update(conn: &mut PooledConn, admin_user_token: &Self) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_admin_user_token)
            .filter(dsl::admin_user_id.eq(admin_user_token.admin_user_id))
            .set(admin_user_token);

        debug_sql!(&query);

        query.execute(conn)
    }
}
