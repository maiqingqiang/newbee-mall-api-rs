use crate::app::admin::{Carousel, CarouselListRequest, DeleteCarouselRequest};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::services;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get};

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
        response.push(Carousel {
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

// 获取单条轮播图信息
#[get("{carousel_id}")]
pub async fn detail(pool: Data<DatabasePool>, carousel_id: Path<i32>) -> result::Response {
    let conn = &mut pool.get()?;

    let carousel = services::carousel::detail(conn, carousel_id.into_inner())?;

    Response::success(Carousel {
        carousel_id: carousel.carousel_id,
        carousel_url: carousel.carousel_url,
        redirect_url: carousel.redirect_url,
        carousel_rank: carousel.carousel_rank,
        is_deleted: carousel.is_deleted,
        create_time: carousel.create_time,
        create_user: carousel.create_user,
        update_time: carousel.create_time,
        update_user: carousel.create_user,
    })
}

// 批量删除轮播图信息
#[delete("")]
pub async fn delete(
    pool: Data<DatabasePool>,
    Json(json): Json<DeleteCarouselRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::carousel::delete(conn, json.carousel_ids)?;

    Response::success(())
}
