use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::models::carousel::Carousel;
use crate::models::pagination::Paginator;

// 商品搜索
pub fn list(
    conn: &mut PooledConn,
    page_number: Option<i64>,
    page_size: Option<i64>,
) -> result::Result<Paginator<Carousel>> {
    Ok(Carousel::list(conn, page_number, page_size)?)
}
