pub mod admin_user;

use serde::{Deserialize};

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LoginRequest {
    #[serde(rename = "userName")]
    pub login_user_name: String,
    #[serde(rename = "passwordMd5")]
    pub login_password: String,
}
