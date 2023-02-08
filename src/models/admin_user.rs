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
