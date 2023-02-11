use crate::bootstrap::database::PooledConn;
use crate::bootstrap::error::ApplicationError;
use crate::bootstrap::result;
use crate::models::admin_user_token::AdminUserToken;
use crate::models::AdminUser;
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
        Ok(mut admin_user_token) => {
            admin_user_token.token = token;
            admin_user_token.update_time = Local::now().naive_local();
            admin_user_token.expire_time = Local::now().add(Duration::days(2)).naive_local();

            AdminUserToken::update(conn, &admin_user_token)?;

            admin_user_token
        }
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

pub fn update_password(
    conn: &mut PooledConn,
    mut admin_user: AdminUser,
    original_password: String,
    new_password: String,
) -> result::Result<()> {
    if original_password == admin_user.login_password {
        admin_user.login_password = new_password;

        if AdminUser::update(conn, &admin_user)? > 0 && AdminUserToken::delete(conn, admin_user.admin_user_id)? > 0 {
            return Ok(());
        }
    }

    Err("database error".into())
}

pub fn update_name(
    conn: &mut PooledConn,
    mut admin_user: AdminUser,
    login_user_name: String,
    nick_name: String,
) -> result::Result<()> {
    admin_user.login_user_name = login_user_name;
    admin_user.nick_name = nick_name;

    if AdminUser::update(conn, &admin_user)? > 0 {
        return Ok(());
    }

    Err("database error".into())
}