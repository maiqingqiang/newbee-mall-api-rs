use crate::app::mall::{UserInfoResponse, LoginRequest, RegisterRequest, EditUserInfoRequest};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::constant::INTRODUCE_SIGN;
use crate::models::user::{NewUser};
use crate::services;
use actix_web::{get, post, put, web};
use chrono::Local;
use crate::middleware::authentication::Identity;
use crate::utils::md5;

// 用户注册
#[post("/register")]
pub async fn register(
    pool: web::Data<DatabasePool>,
    web::Json(data): web::Json<RegisterRequest>,
) -> result::Response {
    let user = NewUser {
        login_name: data.login_name.clone(),
        nick_name: data.login_name.clone(),
        password_md5: md5(data.password.as_str()),
        introduce_sign: INTRODUCE_SIGN,
        create_time: Local::now().naive_local(),
    };

    let conn = &mut pool.get()?;

    services::user::register(conn, user)?;

    Response::success(())
}

// 登录接口
#[post("/login")]
pub async fn login(
    pool: web::Data<DatabasePool>,
    web::Json(data): web::Json<LoginRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;
    let token = services::user::login(conn, data)?;

    Response::success(token)
}

// 登出接口
#[post("/logout")]
pub async fn logout(
    pool: web::Data<DatabasePool>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;
    identity.logout(conn);

    Response::success(())
}

// 获取用户信息
#[get("/info")]
pub async fn info(identity: Identity) -> result::Response {
    Response::success(UserInfoResponse {
        nick_name: identity.user.nick_name,
        login_name: identity.user.login_name,
        introduce_sign: identity.user.introduce_sign,
    })
}

// 修改用户信息
#[put("/info")]
pub async fn edit_info(
    pool: web::Data<DatabasePool>,
    web::Json(data): web::Json<EditUserInfoRequest>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::user::edit_info(conn, identity.user, data)?;

    Response::success(())
}

