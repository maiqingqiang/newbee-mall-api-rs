use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::bootstrap::result;
use crate::constant::DEFAULT_SUCCESS_MESSAGE;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "resultCode")]
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl<T: Serialize> Response<T> {
    pub fn new(status: StatusCode, message: &str, data: T) -> HttpResponse {
        HttpResponse::build(StatusCode::OK).json(Self {
            code: u16::from(status),
            message: message.into(),
            data,
        })
    }
    pub fn success_with_message(data: T, message: &str) -> result::Response {
        Ok(Self::new(StatusCode::OK, message, data))
    }
    pub fn success(data: T) -> result::Response {
        Self::success_with_message(data, DEFAULT_SUCCESS_MESSAGE)
    }
    pub fn ok() -> result::Response {
        Response::<()>::success(())
    }
    pub fn success_with_page(list: Vec<T>, total: i64, current_page: i64, per_page: i64) -> result::Response {
        Response::success(Page::new(list, total, current_page, per_page))
    }
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Page<T> {
    list: Vec<T>,
    total_count: i64,
    total_page: i64,
    curr_page: i64,
    page_size: i64,
}

impl<T> Page<T> {
    pub fn new(list: Vec<T>, total: i64, current_page: i64, per_page: i64) -> Page<T> {
        Self {
            list,
            total_count: total,
            total_page: (total as f64 / per_page as f64).ceil() as i64,
            curr_page: current_page,
            page_size: per_page,
        }
    }
}
