use crate::bootstrap::database::PooledConn;
use crate::debug_sql;
use crate::models::schema::tb_newbee_mall_user_token::dsl;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_user_token)]
pub struct UserToken {
    pub user_id: i64,
    pub token: String,
    pub update_time: NaiveDateTime,
    pub expire_time: NaiveDateTime,
}

impl UserToken {
    pub fn find(conn: &mut PooledConn, id: i64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_user_token.find(id);

        debug_sql!(&query);

        query.first(conn)
    }
    pub fn find_by_token(conn: &mut PooledConn, token: String) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_user_token.filter(dsl::token.eq(token));

        debug_sql!(&query);

        query.first(conn)
    }
    pub fn create(conn: &mut PooledConn, user_token: &UserToken) -> QueryResult<usize> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_user_token).values(user_token);

        debug_sql!(&query);

        query.execute(conn)
    }
    pub fn delete(conn: &mut PooledConn, user_id: i64) -> QueryResult<usize> {
        let query = diesel::delete(dsl::tb_newbee_mall_user_token).filter(dsl::user_id.eq(user_id));

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn update(conn: &mut PooledConn, user_token: &Self) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_user_token)
            .filter(dsl::user_id.eq(user_token.user_id))
            .set(user_token);

        debug_sql!(&query);

        query.execute(conn)
    }
}
