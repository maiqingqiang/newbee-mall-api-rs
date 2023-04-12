pub mod app;
pub mod bootstrap;
pub mod config;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

pub mod constant {
    pub const DEFAULT_PER_PAGE: i64 = 10;
    pub const DEFAULT_PAGE_NUM: i64 = 1;

    pub const TOKEN_LENGTH: i8 = 32;

    pub const INTRODUCE_SIGN: &str = "随新所欲，蜂富多彩";

    pub const DEFAULT_SUCCESS_MESSAGE: &str = "SUCCESS";
    pub const DEFAULT_FAIL_MESSAGE: &str = "FAIL";

    pub const DATA_NOT_EXIST: &str = "未查询到记录！";
    pub const REQUEST_FORBIDEN_ERROR: &str = "禁止该操作！";
    pub const NO_PERMISSION_ERROR: &str = "无权限！";

    pub const SHOPPING_CART_ITEM_LIMIT_NUMBER: i32 = 5;
    pub const SHOPPING_CART_ITEM_TOTAL_NUMBER: i32 = 20;

    pub const FILE_UPLOAD_DIC: &str = "./website/public/upload/";
}
