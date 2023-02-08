use crate::bootstrap::database::PooledConn;
use crate::bootstrap::{result};
use crate::bootstrap::error::ApplicationError;
use crate::models::{goods, Goods, GoodsFilter};
use crate::models::pagination::Paginator;

// 商品搜索
pub fn list_by_search(
    conn: &mut PooledConn,
    filter: &GoodsFilter,
) -> result::Result<Paginator<Goods>> {
    Ok(Goods::get(conn, &filter)?)
}

pub fn find(conn: &mut PooledConn, goods_id: u64) -> result::Result<Goods> {
    let good = Goods::find(conn, goods_id)?;

    if good.goods_sell_status != goods::ON_SHELF {
        return Err(ApplicationError::from("商品已下架！"));
    }

    Ok(good)
}
