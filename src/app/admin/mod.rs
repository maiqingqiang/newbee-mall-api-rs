pub mod admin_user;

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