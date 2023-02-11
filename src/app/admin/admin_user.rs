use crate::app::admin::{LoginRequest, ProfileResponse, UpdateNameRequest, UpdatePasswordRequest};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::middleware::authentication::AdminIdentity;
use crate::services;
use actix_web::web::{Data, Json};
use actix_web::{delete, get, post, put};

// 登录接口
#[post("/login")]
pub async fn login(pool: Data<DatabasePool>, Json(json): Json<LoginRequest>) -> result::Response {
    let conn = &mut pool.get()?;

    let token = services::admin_user::login(conn, json.login_user_name, json.login_password)?;

    Response::success(token)
}

// 获取用户信息
#[get("/profile")]
pub async fn profile(identity: AdminIdentity) -> result::Response {
    Response::success(ProfileResponse {
        admin_user_id: identity.admin_user.admin_user_id,
        login_user_name: identity.admin_user.login_user_name,
        login_password: "******".to_string(),
        nick_name: identity.admin_user.nick_name,
        locked: identity.admin_user.locked,
    })
}

// 获取用户信息
#[put("/password")]
pub async fn update_password(
    pool: Data<DatabasePool>,
    identity: AdminIdentity,
    Json(json): Json<UpdatePasswordRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::admin_user::update_password(conn, identity.admin_user, json.original_password, json.new_password)?;

    Response::success(())
}

// 获取用户信息
#[put("/name")]
pub async fn update_name(
    pool: Data<DatabasePool>,
    identity: AdminIdentity,
    Json(json): Json<UpdateNameRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::admin_user::update_name(conn, identity.admin_user, json.login_user_name, json.nick_name)?;

    Response::success(())
}

// 登出接口
#[delete("/logout")]
pub async fn logout(pool: Data<DatabasePool>, identity: AdminIdentity) -> result::Response {
    let conn = &mut pool.get()?;
    identity.logout(conn);
    Response::success(())
}