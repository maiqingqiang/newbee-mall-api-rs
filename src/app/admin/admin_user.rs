use crate::app::admin::LoginRequest;
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::services;
use actix_web::web::{Data, Json};
use actix_web::{post};

// 登录接口
#[post("/login")]
pub async fn login(pool: Data<DatabasePool>, Json(json): Json<LoginRequest>) -> result::Response {
    let conn = &mut pool.get()?;

    let token = services::admin_user::login(conn, json.login_user_name, json.login_password)?;

    Response::success(token)
}
