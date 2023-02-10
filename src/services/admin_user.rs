use crate::bootstrap::database::PooledConn;
use crate::bootstrap::error::ApplicationError;
use crate::bootstrap::result;
use crate::models::admin_user_token::AdminUserToken;
use crate::models::{AdminUser};
use crate::utils::token::generate_token;
use chrono::{Duration, Local};
use std::ops::Add;

pub fn login(
    conn: &mut PooledConn,
    login_user_name: String,
    login_password: String,
) -> result::Result<String> {
    let admin_user =
        match AdminUser::find_by_login_user_name_password(conn, login_user_name, login_password) {
            Ok(admin_user) => admin_user,
            Err(_) => {
                return Err(ApplicationError::from("登录失败！"));
            }
        };

    let token = generate_token(admin_user.admin_user_id);

    let admin_user_token = match AdminUserToken::find(conn, admin_user.admin_user_id) {
        Ok(admin_user_token) => admin_user_token,
        Err(_) => {
            let admin_user_token = AdminUserToken {
                admin_user_id: admin_user.admin_user_id,
                token: token.clone(),
                update_time: Local::now().naive_local(),
                expire_time: Local::now().add(Duration::days(2)).naive_local(),
            };
            AdminUserToken::create(conn, &admin_user_token)?;

            admin_user_token
        }
    };

    Ok(admin_user_token.token)
}
