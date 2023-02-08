use crate::app::mall::*;
use actix_web::web;

pub fn register_routes(s: &mut web::ServiceConfig) {
    s.service(
        web::scope("/v1")
            .service(goods::search)
            .service(goods_category::categories)
            .service(index::index)
            .service(order::save)
            .service(order::pay_success)

            // /api/v1/user
            .service(web::scope("/user")
                .service(user::register)
                .service(user::login)
                .service(user::logout)
                .service(user::edit_info)
                .service(user::info)
            )

            // /api/v1/goods
            .service(web::scope("/goods")
                .service(goods::detail))

            // /api/v1/address
            .service(web::scope("/address")
                .service(user_address::list)
                .service(user_address::update)
                .service(user_address::save)
                .service(user_address::default)
                .service(user_address::detail)
                .service(user_address::delete)
            )

            // /api/v1/shop-cart
            .service(web::scope("/shop-cart")
                .service(shopping_cart::list)
                .service(shopping_cart::update)
                .service(shopping_cart::save)
                .service(shopping_cart::delete)
                .service(shopping_cart::settle)
            )

            // /api/v1/shop-cart
            .service(web::scope("/order")
                .service(order::list)
                .service(order::detail)
                .service(order::finish)
                .service(order::cancel)
            )
    );
}
