use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::models::goods_category::{GoodsCategory, SecondGoodsCategory, ThirdGoodsCategory};

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
