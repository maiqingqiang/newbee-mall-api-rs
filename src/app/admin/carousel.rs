use crate::app::admin::{CarouselListRequest, CarouselListResponse};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::services;
use actix_web::get;
use actix_web::web::{Data, Query};

// 轮播图列表
#[get("")]
pub async fn list(
    pool: Data<DatabasePool>,
    Query(query): Query<CarouselListRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    let carousels_with_paginator =
        services::carousel::list(conn, query.page_number, query.page_size)?;

    let mut response = vec![];

    for carousel in carousels_with_paginator.data {
        response.push(CarouselListResponse {
            carousel_id: carousel.carousel_id,
            carousel_url: carousel.carousel_url,
            redirect_url: carousel.redirect_url,
            carousel_rank: carousel.carousel_rank,
            is_deleted: carousel.is_deleted,
            create_time: carousel.create_time,
            create_user: carousel.create_user,
            update_time: carousel.update_time,
            update_user: carousel.update_user,
        })
    }

    Response::success_with_page(
        response,
        carousels_with_paginator.total,
        carousels_with_paginator.current_page,
        carousels_with_paginator.per_page,
    )
}
