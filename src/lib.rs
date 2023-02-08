pub mod app;
pub mod bootstrap;
pub mod config;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;
pub mod middleware;

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
}

#[allow(dead_code)]
fn get_order_status_str(status: i8) -> &'static str {
    match Some(status) {
        Some(0) => "待支付",
        Some(1) => "已支付",
        Some(2) => "配货完成",
        Some(3) => "出库成功",
        Some(4) => "交易成功",

        Some(-1) => "手动关闭",
        Some(-2) => "超时关闭",
        Some(-3) => "商家关闭",
        _ => "error"
    }
}