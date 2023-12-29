use crate::bootstrap::database::PooledConn;
use crate::bootstrap::error::ApplicationError;
use crate::bootstrap::result;
use crate::models::goods;
use crate::models::goods::{Goods, GoodsListFilter, GoodsSearchFilter, UpdateGoods};
use crate::models::goods_category::{GoodsCategory, CATEGORY_LEVEL_THIRD};
use crate::models::pagination::Paginator;

// 商品搜索
pub fn list_by_search(
    conn: &mut PooledConn,
    filter: &GoodsSearchFilter,
) -> result::Result<Paginator<Goods>> {
    Ok(Goods::get_by_search(conn, filter)?)
}

pub fn find(conn: &mut PooledConn, goods_id: u64) -> result::Result<Goods> {
    let good = Goods::find(conn, goods_id)?;

    if good.goods_sell_status != goods::ON_SHELF {
        return Err(ApplicationError::from("商品已下架！"));
    }

    Ok(good)
}

// 商品列表
pub fn list(conn: &mut PooledConn, filter: &GoodsListFilter) -> result::Result<Paginator<Goods>> {
    Ok(Goods::get(conn, filter)?)
}

// 修改商品信息接口
pub fn update(conn: &mut PooledConn, update_goods: &UpdateGoods) -> result::Result<()> {
    let goods_carousel = GoodsCategory::find(conn, update_goods.goods_category_id);

    if goods_carousel.is_err() || goods_carousel?.category_level != CATEGORY_LEVEL_THIRD {
        return Err(ApplicationError::from("分类数据异常！"));
    }

    Goods::find(conn, update_goods.goods_id)?;

    match Goods::find_by_category_id_name(
        conn,
        update_goods.goods_category_id,
        update_goods.goods_name.clone(),
    ) {
        Ok(goods) => {
            if goods.goods_id != update_goods.goods_id {
                return Err(ApplicationError::from("已存在相同的商品信息！"));
            }
        }
        Err(_) => {}
    }

    update_goods.update(conn)?;

    Ok(())
}

pub fn detail(conn: &mut PooledConn, goods_id: u64) -> result::Result<Goods> {
    let good = Goods::find(conn, goods_id)?;
    Ok(good)
}
