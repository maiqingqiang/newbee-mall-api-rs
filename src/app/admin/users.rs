use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, put};

use crate::app::admin::{LockUserRequest, User, UserListRequest};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::models::user::UserFilter;
use crate::services;

// 商城注册用户列表
#[get("")]
pub async fn list(
    pool: Data<DatabasePool>,
    Query(json): Query<UserListRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    let users_with_paginator = services::user::list(
        conn,
        UserFilter {
            page_number: json.page_number,
            page_size: json.page_size,
        },
    )?;

    let mut response = vec![];

    for user in users_with_paginator.data {
        response.push(User {
            user_id: user.user_id,
            nick_name: user.nick_name,
            login_name: user.login_name,
            password_md5: user.password_md5,
            introduce_sign: user.introduce_sign,
            is_deleted: user.is_deleted,
            locked_flag: user.locked_flag,
            create_time: user.create_time,
        })
    }

    Response::success_with_page(
        response,
        users_with_paginator.total,
        users_with_paginator.current_page,
        users_with_paginator.per_page,
    )
}

// 用户禁用与解除禁用
#[put("{locked_flag}")]
pub async fn lock_user(
    pool: Data<DatabasePool>,
    Json(json): Json<LockUserRequest>,
    locked_flag: Path<i8>,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::user::lock_user(conn, json.user_ids, locked_flag.into_inner())?;

    Response::success(())
}
