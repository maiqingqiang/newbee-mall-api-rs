use actix_web::{get};
use actix_web::web::{Data, Query};
use crate::app::admin::{User, UserListRequest};
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

    let users_with_paginator = services::user::list(conn, UserFilter {
        page_number: json.page_number,
        page_size: json.page_size,
    })?;

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

    Response::success_with_page(response, users_with_paginator.total, users_with_paginator.current_page, users_with_paginator.per_page)
}