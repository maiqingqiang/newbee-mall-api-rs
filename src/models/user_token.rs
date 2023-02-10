use crate::bootstrap::database::PooledConn;
use crate::models::schema::tb_newbee_mall_user_token::dsl;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_user_token)]
pub struct UserToken {
    pub user_id: i64,
    pub token: String,
    pub update_time: NaiveDateTime,
    pub expire_time: NaiveDateTime,
}

impl UserToken {
    pub fn find(conn: &mut PooledConn, id: i64) -> QueryResult<Self> {
        dsl::tb_newbee_mall_user_token.find(id).first(conn)
    }
    pub fn find_by_token(conn: &mut PooledConn, token: String) -> QueryResult<Self> {
        dsl::tb_newbee_mall_user_token
            .filter(dsl::token.eq(token))
            .first(conn)
    }
    pub fn create(conn: &mut PooledConn, user_token: &UserToken) -> QueryResult<usize> {
        diesel::insert_into(dsl::tb_newbee_mall_user_token)
            .values(user_token)
            .execute(conn)
    }
    pub fn delete(conn: &mut PooledConn, user_id: i64) -> QueryResult<usize> {
        diesel::delete(dsl::tb_newbee_mall_user_token)
            .filter(dsl::user_id.eq(user_id))
            .execute(conn)
    }
}
