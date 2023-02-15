use crate::app::admin::{CategoryListRequest, CategoryListResponse};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::models::goods_category::GoodsCategoryFilter;
use crate::services;
use actix_web::web::Query;
use actix_web::{get, web};

// 商品分类列表接口
#[get("")]
pub async fn list(
    pool: web::Data<DatabasePool>,
    Query(query): Query<CategoryListRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    let categories_with_paginator = services::goods_category::list(
        conn,
        GoodsCategoryFilter {
            page_number: query.page_number,
            page_size: query.page_size,
            category_level: query.category_level,
            parent_id: query.parent_id,
        },
    )?;

    let mut response = vec![];

    for category in categories_with_paginator.data {
        response.push(CategoryListResponse {
            category_id: category.category_id,
            category_level: category.category_level,
            parent_id: category.parent_id,
            category_name: category.category_name,
            category_rank: category.category_rank,
            is_deleted: category.is_deleted,
            create_time: category.create_time,
            create_user: category.create_user,
            update_time: category.update_time,
            update_user: category.update_user,
        })
    }

    Response::success_with_page(
        response,
        categories_with_paginator.total,
        categories_with_paginator.current_page,
        categories_with_paginator.per_page,
    )
}
