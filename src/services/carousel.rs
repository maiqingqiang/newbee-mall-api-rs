use crate::bootstrap::database::PooledConn;
use crate::bootstrap::result;
use crate::models::carousel::{Carousel, NewCarousel};
use crate::models::pagination::Paginator;
use chrono::Local;

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

pub fn create(
    conn: &mut PooledConn,
    admin_user_id: i64,
    carousel_rank: i32,
    carousel_url: String,
    redirect_url: String,
) -> result::Result<()> {
    Carousel::create(
        conn,
        NewCarousel {
            carousel_url,
            redirect_url,
            carousel_rank,
            create_time: Local::now().naive_local(),
            create_user: admin_user_id as i32,
        },
    )?;

    Ok(())
}

pub fn update(
    conn: &mut PooledConn,
    carousel_id: i32,
    carousel_rank: i32,
    carousel_url: String,
    redirect_url: String,
) -> result::Result<()> {
    let mut carousel = Carousel::find(conn, carousel_id)?;

    carousel.carousel_rank = carousel_rank;
    carousel.carousel_url = carousel_url;
    carousel.redirect_url = redirect_url;

    Carousel::update(conn, carousel)?;

    Ok(())
}
