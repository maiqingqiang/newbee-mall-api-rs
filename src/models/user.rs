use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use rand::Rng;

use crate::bootstrap::database::PooledConn;
use crate::models::NOT_DELETE;
use crate::models::schema::tb_newbee_mall_user::dsl;
use crate::utils::md5_string;
use crate::models::schema;

#[derive(Debug, Queryable, Clone, AsChangeset)]
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
    // 未锁定
    pub const NOT_LOCK: i8 = 0;
    // 已锁定
    pub const LOCKED: i8 = 1;

    pub fn create(conn: &mut PooledConn, user: NewUser) -> QueryResult<usize> {
        diesel::insert_into(dsl::tb_newbee_mall_user)
            .values(&user)
            .execute(conn)
    }

    pub fn find(conn: &mut PooledConn, user_id: i64) -> QueryResult<Self> {
        dsl::tb_newbee_mall_user
            .find(user_id)
            .first(conn)
    }

    pub fn find_by_login_name(conn: &mut PooledConn, login_name: String) -> QueryResult<Self> {
        dsl::tb_newbee_mall_user
            .filter(dsl::login_name.eq(login_name))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .first(conn)
    }

    pub fn find_by_login_name_password(conn: &mut PooledConn, login_name: String, password: String) -> QueryResult<Self> {
        dsl::tb_newbee_mall_user
            .filter(dsl::login_name.eq(login_name))
            .filter(dsl::password_md5.eq(password))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .first(conn)
    }

    pub fn generate_token(&self) -> String {
        let s = format!(
            "{}{}{}",
            Local::now().timestamp_millis(),
            self.user_id,
            rand::thread_rng().gen_range(1000..10000)
        );

        md5_string(s)
    }

    pub fn update(conn: &mut PooledConn, user: User) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_user.find(user.user_id))
            .set(&user)
            .execute(conn)
    }
}
