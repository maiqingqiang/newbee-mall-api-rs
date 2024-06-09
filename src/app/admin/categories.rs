use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, post, put, web};
use chrono::Local;

use crate::app::admin::{
    Category, CategoryListRequest, CategoryListResponse, CreateCategoryRequest,
    DeleteCategoryRequest, UpdateCategoryRequest,
};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::middleware::authentication::AdminIdentity;
use crate::models::goods_category::{GoodsCategoryFilter, NewGoodsCategory, UpdateGoodsCategory};
use crate::services;

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
    pool: Data<DatabasePool>,
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

// 批量删除轮播图信息
#[delete("")]
pub async fn delete(
    pool: Data<DatabasePool>,
    Json(json): Json<DeleteCategoryRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::goods_category::delete(conn, json.category_ids)?;

    Response::success(())
}

// 修改分类信息
#[put("")]
pub async fn update(
    pool: Data<DatabasePool>,
    Json(json): Json<UpdateCategoryRequest>,
    identity: AdminIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::goods_category::update(
        conn,
        UpdateGoodsCategory {
            category_id: json.category_id,
            category_level: json.category_level,
            parent_id: json.parent_id,
            category_name: json.category_name,
            category_rank: json.category_rank,
            update_time: Local::now().naive_local(),
            update_user: Some(identity.admin_user.admin_user_id as i32),
        },
    )?;

    Response::success(())
}

// 获取单条轮播图信息
#[get("{category_id}")]
pub async fn detail(pool: Data<DatabasePool>, category_id: Path<i64>) -> result::Response {
    let conn = &mut pool.get()?;

    let goods_category = services::goods_category::detail(conn, category_id.into_inner())?;

    Response::success(Category {
        category_id: goods_category.category_id,
        category_level: goods_category.category_level,
        parent_id: goods_category.parent_id,
        category_name: goods_category.category_name,
        category_rank: goods_category.category_rank,
        is_deleted: goods_category.is_deleted,
        create_time: goods_category.create_time,
        create_user: goods_category.create_user,
        update_time: goods_category.update_time,
        update_user: goods_category.update_user,
    })
}
