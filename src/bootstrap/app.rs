use crate::{config, routes};
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;

pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(move || {
        let data = web::Data::new(super::database::connection());

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .app_data(web::Data::clone(&data))
            .service(
                web::scope("/api")
                    .wrap(crate::middleware::authentication::Authentication)
                    .configure(routes::mall::register_routes),
            )
            .service(web::scope("/manage-api").configure(routes::admin::register_routes))
    })
    .bind((config::APP.host.as_str(), config::APP.port))?
    .run()
    .await
}
