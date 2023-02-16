use chrono::{Local, NaiveDateTime};
use diesel::prelude::*;
use serde::Serialize;

use crate::bootstrap::database::PooledConn;
use crate::debug_sql;
use crate::models::pagination::{Paginate, Paginator};
use crate::models::schema::tb_newbee_mall_goods_category::dsl;
use crate::models::{DELETED, NOT_DELETE};

pub const CATEGORY_LEVEL_FIRST: i8 = 1;
pub const CATEGORY_LEVEL_SECOND: i8 = 2;
pub const CATEGORY_LEVEL_THIRD: i8 = 3;

// 商品分类
#[derive(Debug, Queryable, Serialize, Identifiable)]
#[diesel(primary_key(category_id))]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_goods_category)]
pub struct GoodsCategory {
    // 分类id
    pub category_id: i64,
    // 分类级别(1-一级分类 2-二级分类 3-三级分类)
    pub category_level: i8,
    // 父分类id
    pub parent_id: i64,
    // 分类名称
    pub category_name: String,
    // 排序值(字段越大越靠前)
    pub category_rank: i32,
    // 删除标识字段(0-未删除 1-已删除)
    pub is_deleted: i8,
    // 创建时间
    pub create_time: NaiveDateTime,
    // 创建者id
    pub create_user: i32,
    // 修改时间
    pub update_time: NaiveDateTime,
    // 修改者id
    pub update_user: Option<i32>,
}

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[diesel(primary_key(category_id))]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_goods_category)]
#[diesel(belongs_to(GoodsCategory, foreign_key = parent_id))]
pub struct SecondGoodsCategory {
    // 分类id
    pub category_id: i64,
    // 分类级别(1-一级分类 2-二级分类 3-三级分类)
    pub category_level: i8,
    // 父分类id
    pub parent_id: i64,
    // 分类名称
    pub category_name: String,
    // 排序值(字段越大越靠前)
    pub category_rank: i32,
    // 删除标识字段(0-未删除 1-已删除)
    pub is_deleted: i8,
    // 创建时间
    pub create_time: NaiveDateTime,
    // 创建者id
    pub create_user: i32,
    // 修改时间
    pub update_time: NaiveDateTime,
    // 修改者id
    pub update_user: Option<i32>,
}

#[derive(Debug, Queryable, Serialize, Identifiable, Associations)]
#[diesel(primary_key(category_id))]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_goods_category)]
#[diesel(belongs_to(SecondGoodsCategory, foreign_key = parent_id))]
pub struct ThirdGoodsCategory {
    // 分类id
    pub category_id: i64,
    // 分类级别(1-一级分类 2-二级分类 3-三级分类)
    pub category_level: i8,
    // 父分类id
    pub parent_id: i64,
    // 分类名称
    pub category_name: String,
    // 排序值(字段越大越靠前)
    pub category_rank: i32,
    // 删除标识字段(0-未删除 1-已删除)
    pub is_deleted: i8,
    // 创建时间
    pub create_time: NaiveDateTime,
    // 创建者id
    pub create_user: i32,
    // 修改时间
    pub update_time: NaiveDateTime,
    // 修改者id
    pub update_user: Option<i32>,
}

pub struct GoodsCategoryFilter {
    pub page_number: Option<i64>,
    pub page_size: Option<i64>,
    pub category_level: i8,
    pub parent_id: i64,
}

impl GoodsCategory {
    pub fn collect(
        conn: &mut PooledConn,
    ) -> QueryResult<Vec<(Self, Vec<(SecondGoodsCategory, Vec<ThirdGoodsCategory>)>)>> {
        let query = dsl::tb_newbee_mall_goods_category
            .filter(dsl::category_level.eq(CATEGORY_LEVEL_FIRST))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .order(dsl::category_rank.desc())
            .limit(10);

        debug_sql!(&query);

        let first_categorys = query.load::<Self>(conn)?;

        let query = SecondGoodsCategory::belonging_to(&first_categorys)
            .filter(dsl::category_level.eq(CATEGORY_LEVEL_SECOND))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .order(dsl::category_rank.desc());

        debug_sql!(&query);

        let second_categorys = query.load::<SecondGoodsCategory>(conn)?;

        let query = ThirdGoodsCategory::belonging_to(&second_categorys)
            .filter(dsl::category_level.eq(CATEGORY_LEVEL_THIRD))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .order(dsl::category_rank.desc());

        debug_sql!(&query);

        let third_categorys = query.load::<ThirdGoodsCategory>(conn)?;

        let third_categorys = third_categorys.grouped_by(&second_categorys);

        let second_categorys = second_categorys
            .into_iter()
            .zip(third_categorys)
            .collect::<Vec<_>>();

        let second_categorys = second_categorys.grouped_by(&first_categorys);

        Ok(first_categorys.into_iter().zip(second_categorys).collect())
    }

    pub fn list(
        conn: &mut PooledConn,
        filter: GoodsCategoryFilter,
    ) -> QueryResult<Paginator<Self>> {
        Paginate::new(
            || {
                let mut query = dsl::tb_newbee_mall_goods_category.into_boxed();

                if filter.category_level != 0 {
                    query = query.filter(dsl::category_level.eq(filter.category_level))
                }

                query
                    .filter(dsl::parent_id.eq(filter.parent_id))
                    .filter(dsl::is_deleted.eq(NOT_DELETE))
                    .order(dsl::category_rank.desc())
            },
            filter.page_number,
        )
        .per_page(filter.page_size)
        .load_with_paginator(conn)
    }

    pub fn delete(conn: &mut PooledConn, category_ids: Vec<i64>) -> QueryResult<usize> {
        let query = diesel::update(
            dsl::tb_newbee_mall_goods_category.filter(dsl::category_id.eq_any(category_ids)),
        )
        .set((
            dsl::is_deleted.eq(DELETED),
            dsl::update_time.eq(Local::now().naive_local()),
        ));

        debug_sql!(&query);

        query.execute(conn)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_goods_category)]
pub struct NewGoodsCategory {
    pub category_id: i64,
    pub category_level: i8,
    pub parent_id: i64,
    pub category_name: String,
    pub category_rank: i32,
    pub create_time: NaiveDateTime,
    pub create_user: i32,
}

impl NewGoodsCategory {
    pub fn create(self, conn: &mut PooledConn) -> QueryResult<GoodsCategory> {
        let query = diesel::insert_into(dsl::tb_newbee_mall_goods_category).values(self);

        debug_sql!(&query);

        query.execute(conn)?;

        let query = dsl::tb_newbee_mall_goods_category.find(super::functions::last_insert_id());

        debug_sql!(&query);

        query.first(conn)
    }
}
