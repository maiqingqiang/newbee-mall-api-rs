use crate::app::mall::{Carousel, IndexGoods, IndexResponse};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::models::goods::Goods;
use crate::services;
use actix_web::{get, web};

// 获取首页数据
// 轮播图、新品、推荐等
#[get("/index-infos")]
pub async fn index(pool: web::Data<DatabasePool>) -> result::Response {
    let conn = &mut pool.get()?;

    let (carousel_v, hot_good_v, new_good_v, recommond_good_v) = services::index::index_info(conn)?;

    let mut carousels: Vec<Carousel> = vec![];

    for carousel in carousel_v {
        carousels.push(Carousel {
            carousel_url: carousel.carousel_url,
            redirect_url: carousel.redirect_url,
        })
    }

    let hot_goodses = get_goods(hot_good_v);
    let new_goodses = get_goods(new_good_v);
    let recommend_goodses = get_goods(recommond_good_v);

    Response::success(IndexResponse {
        carousels,
        hot_goodses,
        new_goodses,
        recommend_goodses,
    })
}

fn get_goods(goods: Vec<Goods>) -> Vec<IndexGoods> {
    let mut new_goods: Vec<IndexGoods> = vec![];
    for good in goods {
        new_goods.push(IndexGoods {
            goods_id: good.goods_id,
            goods_name: good.goods_name,
            goods_intro: good.goods_intro,
            goods_cover_img: good.goods_cover_img,
            selling_price: good.selling_price,
            tag: good.tag,
        })
    }
    new_goods
}
