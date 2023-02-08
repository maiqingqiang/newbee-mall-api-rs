use std::ops::Add;
use chrono::{Duration, Local};
use crate::app::mall::{EditUserInfoRequest, LoginRequest};
use crate::bootstrap::database::PooledConn;
use crate::bootstrap::error::ApplicationError;
use crate::bootstrap::result;
use crate::models::user::{NewUser, User};
use crate::models::user_token::UserToken;

pub fn register(conn: &mut PooledConn, user: NewUser) -> result::Result<usize> {
    return match User::find_by_login_name(conn, user.login_name.clone()) {
        Ok(_) => Err("用户名已存在！".into()),
        Err(_) => Ok(User::create(conn, user)?),
    };
}

pub fn login(conn: &mut PooledConn, data: LoginRequest) -> result::Result<String> {
    let user = match User::find_by_login_name_password(conn, data.login_name, data.password_md5) {
        Ok(user) => {
            user
        }
        Err(_) => {
            return Err(ApplicationError::from("登录失败！"));
        }
    };

    if user.locked_flag == User::LOCKED {
        return Err("用户已被禁止登录！".into());
    }

    let token = user.generate_token();

    let user_token = match UserToken::find(conn, user.user_id) {
        Ok(user_token) => {
            user_token
        }
        Err(_) => {
            let user_token = UserToken {
                user_id: user.user_id,
                token: token.clone(),
                update_time: Local::now().naive_local(),
                expire_time: Local::now().add(Duration::days(2)).naive_local(),
            };
            UserToken::create(conn, &user_token)?;

            user_token
        }
    };

    Ok(user_token.token)
}

pub fn edit_info(conn: &mut PooledConn, mut user: User, data: EditUserInfoRequest) -> result::Result<usize> {
    user.nick_name = data.nick_name;
    user.introduce_sign = data.introduce_sign;

    if let Some(password_md5) = data.password_md5 {
        if !password_md5.is_empty() {
            user.password_md5 = password_md5;
        }
    }

    Ok(User::update(conn, user)?)
}