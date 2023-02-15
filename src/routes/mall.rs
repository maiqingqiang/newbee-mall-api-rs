use crate::app::mall::*;
use actix_web::web;

pub fn register_routes(s: &mut web::ServiceConfig) {
    s.service(
        // /api/v1/
        web::scope("/v1")
            .service(goods::search)
            .service(categories::categories)
            .service(index::index)
            .service(order::save)
            .service(order::pay_success)
            // /api/v1/user
            .service(
                web::scope("/user")
                    .service(user::register)
                    .service(user::login)
                    .service(user::logout)
                    .service(user::edit_info)
                    .service(user::info),
            )
            // /api/v1/goods
            .service(web::scope("/goods").service(goods::detail))
            // /api/v1/address
            .service(
                web::scope("/address")
                    .service(address::list)
                    .service(address::update)
                    .service(address::save)
                    .service(address::default)
                    .service(address::detail)
                    .service(address::delete),
            )
            // /api/v1/shop-cart
            .service(
                web::scope("/shop-cart")
                    .service(shop_cart::list)
                    .service(shop_cart::list_by_page)
                    .service(shop_cart::update)
                    .service(shop_cart::save)
                    .service(shop_cart::delete)
                    .service(shop_cart::settle),
            )
            // /api/v1/shop-cart
            .service(
                web::scope("/order")
                    .service(order::list)
                    .service(order::detail)
                    .service(order::finish)
                    .service(order::cancel),
            ),
    );
}
