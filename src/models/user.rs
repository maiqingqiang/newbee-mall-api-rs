use crate::bootstrap::database::PooledConn;
use crate::debug_sql;
use crate::models::schema;
use crate::models::schema::tb_newbee_mall_user::dsl;
use crate::models::NOT_DELETE;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Clone, AsChangeset, Serialize)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_user)]
pub struct User {
    pub user_id: i64,
    pub nick_name: String,
    pub login_name: String,
    pub password_md5: String,
    pub introduce_sign: String,
    pub is_deleted: i8,
    pub locked_flag: i8,
    pub create_time: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::tb_newbee_mall_user)]
pub struct NewUser<'a> {
    pub login_name: String,
    pub nick_name: String,
    pub password_md5: String,
    pub introduce_sign: &'a str,
    pub create_time: NaiveDateTime,
}

impl User {
    pub fn create(conn: &mut PooledConn, user: NewUser) -> QueryResult<usize> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_user).values(&user);

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn find(conn: &mut PooledConn, user_id: i64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_user.find(user_id);

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn find_by_login_name(conn: &mut PooledConn, login_name: String) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_user
            .filter(dsl::login_name.eq(login_name))
            .filter(dsl::is_deleted.eq(NOT_DELETE));

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn find_by_login_name_password(
        conn: &mut PooledConn,
        login_name: String,
        password_md5: String,
    ) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_user
            .filter(dsl::login_name.eq(login_name))
            .filter(dsl::password_md5.eq(password_md5))
            .filter(dsl::is_deleted.eq(NOT_DELETE));

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn update(conn: &mut PooledConn, user: User) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_user.find(user.user_id)).set(&user);

        debug_sql!(&query);

        query.execute(conn)
    }
}
