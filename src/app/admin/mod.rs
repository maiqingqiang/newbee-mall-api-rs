pub mod admin_user;
pub mod carousel;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LoginRequest {
    #[serde(rename = "userName")]
    pub login_user_name: String,
    #[serde(rename = "passwordMd5")]
    pub login_password: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct ProfileResponse {
    #[serde(rename = "adminUserId")]
    pub admin_user_id: i64,
    #[serde(rename = "loginUserName")]
    pub login_user_name: String,
    #[serde(rename = "loginPassword")]
    pub login_password: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    pub locked: Option<i8>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdatePasswordRequest {
    pub new_password: String,
    pub original_password: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateNameRequest {
    #[serde(rename = "loginUserName")]
    pub login_user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CarouselListRequest {
    pub page_number: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Carousel {
    pub carousel_id: i32,
    pub carousel_url: String,
    pub redirect_url: String,
    pub carousel_rank: i32,
    pub is_deleted: i8,
    pub create_time: NaiveDateTime,
    pub create_user: i32,
    pub update_time: NaiveDateTime,
    pub update_user: i32,
}
