pub mod app;
pub mod database;
pub mod error;
pub mod response;

pub mod result {
    use crate::bootstrap::error::ApplicationError;
    use actix_web::HttpResponse;

    pub type Result<T, E = ApplicationError> = std::result::Result<T, E>;

    pub type Response = Result<HttpResponse>;
}
