use crate::constant::FILE_UPLOAD_DIC;
use crate::{config, routes};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;

pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    HttpServer::new(move || {
        let data = web::Data::new(super::database::connection());

        App::new()
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::clone(&data))
            .service(Files::new("/upload", FILE_UPLOAD_DIC).show_files_listing())
            .service(Files::new("/goods-img", FILE_UPLOAD_DIC).show_files_listing())
            .service(
                web::scope("/api")
                    .wrap(crate::middleware::authentication::MallAuthentication)
                    .configure(routes::mall::register_routes),
            )
            .service(
                web::scope("/manage-api")
                    .wrap(crate::middleware::authentication::AdminAuthentication)
                    .configure(routes::admin::register_routes),
            )
    })
    .bind((config::APP.host.as_str(), config::APP.port))?
    .run()
    .await
}
