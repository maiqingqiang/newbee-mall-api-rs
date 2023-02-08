use newbee_mall_api_rs::bootstrap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bootstrap::app::start().await
}
