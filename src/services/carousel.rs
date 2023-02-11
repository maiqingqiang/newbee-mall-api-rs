use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::models::carousel::Carousel;
use crate::models::pagination::Paginator;

pub fn list(
    conn: &mut PooledConn,
    page_number: Option<i64>,
    page_size: Option<i64>,
) -> result::Result<Paginator<Carousel>> {
    Ok(Carousel::list(conn, page_number, page_size)?)
}

pub fn detail(conn: &mut PooledConn, carousel_id: i32) -> result::Result<Carousel> {
    Ok(Carousel::find(conn, carousel_id)?)
}

pub fn delete(conn: &mut PooledConn, carousel_ids: Vec<i32>) -> result::Result<()> {
    Carousel::delete(conn, carousel_ids)?;

    Ok(())
}
