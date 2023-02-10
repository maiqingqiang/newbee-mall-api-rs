use crate::bootstrap::database::PooledConn;
use crate::models::schema::tb_newbee_mall_admin_user::dsl;
use crate::models::{NOT_LOCK};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct AdminUser {
    pub admin_user_id: i64,
    pub login_user_name: String,
    pub login_password: String,
    pub nick_name: String,
    pub locked: Option<i8>,
}

impl AdminUser {
    pub fn find(conn: &mut PooledConn, admin_user_id: i64) -> QueryResult<Self> {
        dsl::tb_newbee_mall_admin_user
            .find(admin_user_id)
            .first(conn)
    }

    pub fn find_by_login_user_name_password(
        conn: &mut PooledConn,
        login_user_name: String,
        login_password: String,
    ) -> QueryResult<Self> {
        dsl::tb_newbee_mall_admin_user
            .filter(dsl::login_user_name.eq(login_user_name))
            .filter(dsl::login_password.eq(login_password))
            .filter(dsl::locked.eq(NOT_LOCK))
            .first(conn)
    }
}
