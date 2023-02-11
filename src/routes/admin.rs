use crate::app::admin::*;
use actix_web::web;

pub fn register_routes(s: &mut web::ServiceConfig) {
    s.service(
        // /api/v1/
        web::scope("/v1")
            // /api/v1/adminUser
            .service(
                web::scope("/adminUser")
                    .service(admin_user::login)
                    .service(admin_user::profile)
                    .service(admin_user::update_password),
            ),
    );
}
