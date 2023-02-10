use crate::app::mall::{
    ShoppingCartListRequest, ShoppingCartSaveRequest, ShoppingCartSettleRequest,
    ShoppingCartUpdateRequest,
};
use crate::bootstrap::{database::DatabasePool, response::Response, result};
use crate::middleware::authentication::Identity;
use crate::models::shopping_cart::NewShoppingCart;
use crate::services::shopping_cart;
use actix_web::{
    get, post, put,
    web::{Data, Json, Query, Path},
};

// 购物车列表(每页默认5条)
#[get("/page")]
pub async fn list_by_page(
    pool: Data<DatabasePool>,
    identity: Identity,
    Query(query): Query<ShoppingCartListRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    let shopping_carts_with_paginator =
        shopping_cart::list_with_page(conn, identity.user.user_id, query.page_number)?;

    let shopping_cart_items =
        shopping_cart::to_shopping_cart_items(conn, shopping_carts_with_paginator.data)?;

    Response::success_with_page(
        shopping_cart_items,
        shopping_carts_with_paginator.total,
        shopping_carts_with_paginator.current_page,
        shopping_carts_with_paginator.per_page,
    )
}

// 购物车列表(网页移动端不分页)
#[get("")]
pub async fn list(pool: Data<DatabasePool>, identity: Identity) -> result::Response {
    let conn = &mut pool.get()?;

    let response = shopping_cart::list(conn, identity.user.user_id)?;

    Response::success(response)
}

// 添加商品到购物车接口
#[post("")]
pub async fn save(
    pool: Data<DatabasePool>,
    Json(data): Json<ShoppingCartSaveRequest>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    shopping_cart::save(
        conn,
        NewShoppingCart {
            user_id: identity.user.user_id,
            goods_id: data.goods_id,
            goods_count: data.goods_count,
        },
    )?;

    Response::success(())
}

// 添加商品到购物车接口
#[put("")]
pub async fn update(
    pool: Data<DatabasePool>,
    Json(data): Json<ShoppingCartUpdateRequest>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    shopping_cart::update(
        conn,
        identity.user.user_id,
        data.cart_item_id,
        data.goods_count,
    )?;

    Response::success(())
}

// 删除购物项
#[put("/{newBeeMallShoppingCartItemId}")]
pub async fn delete(
    pool: Data<DatabasePool>,
    path: Path<(i64, )>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let cart_item_id = path.into_inner().0;

    shopping_cart::delete(conn, identity.user.user_id, cart_item_id)?;

    Response::success(())
}

// 根据购物项id数组查询购物项明细
#[get("/settle")]
pub async fn settle(
    pool: Data<DatabasePool>,
    Query(query): Query<ShoppingCartSettleRequest>,
    identity: Identity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let cart_item_ids = query
        .cart_item_ids
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    Response::success(shopping_cart::settle(
        conn,
        identity.user.user_id,
        cart_item_ids,
    )?)
}
