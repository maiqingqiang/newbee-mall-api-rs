use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::models::goods_category::{
    GoodsCategory, GoodsCategoryFilter, NewGoodsCategory, SecondGoodsCategory, ThirdGoodsCategory,
};
use crate::models::pagination::Paginator;

pub fn collect(
    conn: &mut PooledConn,
) -> result::Result<
    Vec<(
        GoodsCategory,
        Vec<(SecondGoodsCategory, Vec<ThirdGoodsCategory>)>,
    )>,
> {
    let categories = GoodsCategory::collect(conn)?;

    Ok(categories)
}

pub fn list(
    conn: &mut PooledConn,
    filter: GoodsCategoryFilter,
) -> result::Result<Paginator<GoodsCategory>> {
    Ok(GoodsCategory::list(conn, filter)?)
}

pub fn create(
    conn: &mut PooledConn,
    goods_category: NewGoodsCategory,
) -> result::Result<GoodsCategory> {
    Ok(goods_category.create(conn)?)
}

pub fn delete(conn: &mut PooledConn, category_ids: Vec<i64>) -> result::Result<()> {
    GoodsCategory::delete(conn, category_ids)?;

    Ok(())
}
