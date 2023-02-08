use actix_web::{get, post, put, web};
use crate::app::mall::{ShoppingCartSaveRequest, ShoppingCartSettleRequest, ShoppingCartUpdateRequest};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::middleware::authentication::Identity;
use crate::models::shopping_cart::{NewShoppingCart};
use crate::services;

// 购物车列表(每页默认5条)
#[get("/page")]
pub async fn list_by_page() -> result::Response {
    todo!()
}

// 购物车列表(网页移动端不分页)
#[get("")]
pub async fn list(
    pool: web::Data<DatabasePool>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let response = services::shopping_cart::list(conn, identity.user.user_id)?;

    Response::success(response)
}

// 添加商品到购物车接口
#[post("")]
pub async fn save(
    pool: web::Data<DatabasePool>,
    web::Json(data): web::Json<ShoppingCartSaveRequest>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::shopping_cart::save(conn, NewShoppingCart {
        user_id: identity.user.user_id,
        goods_id: data.goods_id,
        goods_count: data.goods_count,
    })?;

    Response::success(())
}

// 添加商品到购物车接口
#[put("")]
pub async fn update(
    pool: web::Data<DatabasePool>,
    web::Json(data): web::Json<ShoppingCartUpdateRequest>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::shopping_cart::update(conn, identity.user.user_id, data.cart_item_id, data.goods_count)?;

    Response::success(())
}

// 删除购物项
#[put("/{newBeeMallShoppingCartItemId}")]
pub async fn delete(
    pool: web::Data<DatabasePool>,
    path: web::Path<(i64, )>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let cart_item_id = path.into_inner().0;

    services::shopping_cart::delete(conn, identity.user.user_id, cart_item_id)?;

    Response::success(())
}

// 根据购物项id数组查询购物项明细
#[get("/settle")]
pub async fn settle(
    pool: web::Data<DatabasePool>,
    web::Query(query): web::Query<ShoppingCartSettleRequest>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let cart_item_ids = query.cart_item_ids
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    Response::success(services::shopping_cart::settle(conn, identity.user.user_id, cart_item_ids)?)
}