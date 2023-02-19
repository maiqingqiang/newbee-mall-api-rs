use crate::app::mall::{GoodsDetailResponse, GoodsSearchRequest, GoodsSearchResponse};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::models::goods::GoodsSearchFilter;
use crate::services;
use actix_web::{get, web};

// 商品搜索接口
// 根据关键字和分类id进行搜索
#[get("/search")]
pub async fn search(
    pool: web::Data<DatabasePool>,
    web::Query(query): web::Query<GoodsSearchRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    let goods_with_paginator = services::goods::list_by_search(
        conn,
        &GoodsSearchFilter {
            goods_category_id: query.goods_category_id,
            keyword: query.keyword,
            order_by: query.order_by,
            page_number: query.page_number,
        },
    )?;

    let mut response: Vec<GoodsSearchResponse> = vec![];

    for good in goods_with_paginator.data {
        response.push(GoodsSearchResponse {
            goods_id: good.goods_id,
            goods_name: good.goods_name,
            goods_intro: good.goods_intro,
            goods_cover_img: good.goods_cover_img,
            selling_price: good.selling_price,
        })
    }

    Response::success_with_page(
        response,
        goods_with_paginator.total,
        goods_with_paginator.current_page,
        goods_with_paginator.per_page,
    )
}

// 商品详情接口
// 传参为商品id
#[get("/detail/{goods_id}")]
pub async fn detail(pool: web::Data<DatabasePool>, goods_id: web::Path<u64>) -> result::Response {
    let mut pool = pool.get()?;

    let good = services::goods::find(&mut pool, goods_id.into_inner())?;

    let response = GoodsDetailResponse {
        goods_id: good.goods_id,
        goods_name: good.goods_name,
        goods_intro: good.goods_intro,
        goods_cover_img: good.goods_cover_img,
        selling_price: good.selling_price,
        goods_detail_content: good.goods_detail_content,
        original_price: good.original_price,
        tag: good.tag,
        goods_carousel_list: good.goods_carousel.split('，').collect(),
    };

    Response::success(response)
}
