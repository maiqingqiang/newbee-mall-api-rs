use crate::app::admin::{Goods, GoodsListRequest};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::models::goods::GoodsListFilter;
use crate::services;
use actix_web::get;
use actix_web::web::{Data, Query};

// 用户禁用与解除禁用
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
