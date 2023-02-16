use crate::app::admin::{CategoryListRequest, CategoryListResponse, CreateCategoryRequest};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::middleware::authentication::AdminIdentity;
use crate::models::goods_category::{GoodsCategoryFilter, NewGoodsCategory};
use crate::services;
use actix_web::web::{Json, Query};
use actix_web::{get, post, web};
use chrono::Local;

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

// 新增分类接口
#[post("")]
pub async fn create(
    pool: web::Data<DatabasePool>,
    Json(json): Json<CreateCategoryRequest>,
    identity: AdminIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let goods_category = services::goods_category::create(
        conn,
        NewGoodsCategory {
            category_id: json.parent_id,
            category_level: json.category_level,
            parent_id: json.parent_id,
            category_name: json.category_name,
            category_rank: json.category_rank,
            create_time: Local::now().naive_local(),
            create_user: identity.admin_user.admin_user_id as i32,
        },
    )?;

    Response::success(goods_category)
}
