use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::models::carousel::Carousel;
use crate::models::index_config::{
    IndexConfig, INDEX_GOODS_HOT, INDEX_GOODS_NEW, INDEX_GOODS_RECOMMOND,
};
use crate::models::Goods;

// 商品搜索
pub fn index_info(
    conn: &mut PooledConn,
) -> result::Result<(Vec<Carousel>, Vec<Goods>, Vec<Goods>, Vec<Goods>)> {
    // 轮播图
    let carousels = Carousel::list(conn, 5)?;

    // 热销商品
    let good_ids = get_good_ids(conn, INDEX_GOODS_HOT, 4)?;
    let hot_goods = Goods::get_by_goods_ids(conn, good_ids)?;

    // 新品
    let good_ids = get_good_ids(conn, INDEX_GOODS_NEW, 5)?;
    let new_goods = Goods::get_by_goods_ids(conn, good_ids)?;

    // 推荐商品
    let good_ids = get_good_ids(conn, INDEX_GOODS_RECOMMOND, 5)?;
    let recommond_goods = Goods::get_by_goods_ids(conn, good_ids)?;

    Ok((carousels, hot_goods, new_goods, recommond_goods))
}

fn get_good_ids(conn: &mut PooledConn, config_type: i8, limit: i64) -> result::Result<Vec<u64>> {
    let configs = IndexConfig::list(conn, config_type, limit)?;
    let mut good_ids: Vec<u64> = vec![];
    for config in configs {
        good_ids.push(config.goods_id as u64)
    }
    Ok(good_ids)
}
