use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::bootstrap::database::PooledConn;
use crate::debug_sql;
use crate::models::schema::tb_newbee_mall_admin_user::dsl;
use crate::models::NOT_LOCK;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_admin_user)]
pub struct AdminUser {
    pub admin_user_id: i64,
    pub login_user_name: String,
    pub login_password: String,
    pub nick_name: String,
    pub locked: Option<i8>,
}

impl AdminUser {
    pub fn find(conn: &mut PooledConn, admin_user_id: i64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_admin_user.find(admin_user_id);

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn find_by_login_user_name_password(
        conn: &mut PooledConn,
        login_user_name: String,
        login_password: String,
    ) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_admin_user
            .filter(dsl::login_user_name.eq(login_user_name))
            .filter(dsl::login_password.eq(login_password))
            .filter(dsl::locked.eq(NOT_LOCK));

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn update(conn: &mut PooledConn, admin_user: &Self) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_admin_user)
            .filter(dsl::admin_user_id.eq(admin_user.admin_user_id))
            .set(admin_user);

        debug_sql!(&query);

        query.execute(conn)
    }
}
