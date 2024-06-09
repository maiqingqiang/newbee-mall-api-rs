use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, post, put};
use chrono::Local;

use crate::app::admin::{
    CreateGoodRequest, Goods, GoodsDetailResponse, GoodsListRequest, UpdateGoodsRequest,
};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::middleware::authentication::AdminIdentity;
use crate::models::goods::{GoodsListFilter, NewGood, UpdateGoods};
use crate::services;

// 商品列表接口
#[get("list")]
pub async fn list(
    pool: Data<DatabasePool>,
    Query(query): Query<GoodsListRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    let goods_with_paginator = services::goods::list(
        conn,
        &GoodsListFilter {
            page_number: query.page_number,
            page_size: query.page_size,
            goods_name: query.goods_name,
            goods_sell_status: query.goods_sell_status,
        },
    )?;

    let mut response: Vec<Goods> = vec![];

    for good in goods_with_paginator.data {
        response.push(Goods {
            goods_id: good.goods_id,
            goods_name: good.goods_name,
            goods_intro: good.goods_intro,
            goods_category_id: good.goods_category_id,
            goods_cover_img: good.goods_cover_img,
            goods_carousel: good.goods_carousel,
            original_price: good.original_price,
            selling_price: good.selling_price,
            stock_num: good.stock_num,
            tag: good.tag,
            goods_sell_status: good.goods_sell_status,
            create_user: good.create_user,
            create_time: good.create_time,
            update_user: good.update_user,
            update_time: good.update_time,
            goods_detail_content: good.goods_detail_content,
        })
    }

    Response::success_with_page(
        response,
        goods_with_paginator.total,
        goods_with_paginator.current_page,
        goods_with_paginator.per_page,
    )
}

// 修改商品信息接口
#[put("")]
pub async fn update(
    pool: Data<DatabasePool>,
    Json(goods): Json<UpdateGoodsRequest>,
    identity: AdminIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let goods = services::goods::update(
        conn,
        &UpdateGoods {
            goods_id: goods.goods_id,
            goods_name: goods.goods_name,
            goods_intro: goods.goods_intro,
            goods_category_id: goods.goods_category_id,
            goods_cover_img: goods.goods_cover_img,
            original_price: goods.original_price,
            selling_price: goods.selling_price,
            stock_num: goods.stock_num,
            tag: goods.tag,
            goods_sell_status: goods.goods_sell_status,
            update_user: identity.admin_user.admin_user_id as i32,
            goods_detail_content: goods.goods_detail_content,
            update_time: Local::now().naive_local(),
        },
    )?;

    Response::success(goods)
}

// 获取单条商品信息接口
#[get("{goods_id}")]
pub async fn detail(pool: Data<DatabasePool>, goods_id: Path<u64>) -> result::Response {
    let conn = &mut pool.get()?;

    let goods = services::goods::detail(conn, goods_id.into_inner())?;

    Response::success(GoodsDetailResponse {
        goods: Goods {
            goods_id: goods.goods_id,
            goods_name: goods.goods_name,
            goods_intro: goods.goods_intro,
            goods_category_id: goods.goods_category_id,
            goods_cover_img: goods.goods_cover_img,
            goods_carousel: goods.goods_carousel,
            original_price: goods.original_price,
            selling_price: goods.selling_price,
            stock_num: goods.stock_num,
            tag: goods.tag,
            goods_sell_status: goods.goods_sell_status,
            create_user: goods.create_user,
            create_time: goods.create_time,
            update_user: goods.update_user,
            update_time: goods.update_time,
            goods_detail_content: goods.goods_detail_content,
        },
    })
}

// 新增商品信息接口
#[post("")]
pub async fn create(
    pool: Data<DatabasePool>,
    Json(goods): Json<CreateGoodRequest>,
    identity: AdminIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let goods = services::goods::create(
        conn,
        NewGood {
            goods_category_id: goods.goods_category_id,
            goods_cover_img: goods.goods_cover_img,
            goods_detail_content: goods.goods_detail_content,
            goods_intro: goods.goods_intro,
            goods_name: goods.goods_name,
            goods_sell_status: goods.goods_sell_status,
            original_price: goods.original_price,
            selling_price: goods.selling_price,
            stock_num: goods.stock_num,
            tag: goods.tag,
            create_user: identity.admin_user.admin_user_id as i32,
            create_time: Local::now().naive_local(),
        },
    )?;

    Response::success(goods)
}
