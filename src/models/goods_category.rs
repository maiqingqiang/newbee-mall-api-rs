use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::bootstrap::database::PooledConn;
use crate::models::schema::tb_newbee_mall_goods_category::dsl::tb_newbee_mall_goods_category;
use crate::models::schema::tb_newbee_mall_goods_category::{
    category_level, category_rank, is_deleted,
};
use crate::models::NOT_DELETE;

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

impl GoodsCategory {
    pub fn collect(
        conn: &mut PooledConn,
    ) -> QueryResult<Vec<(Self, Vec<(SecondGoodsCategory, Vec<ThirdGoodsCategory>)>)>> {
        let first_categorys = Self::list(conn, CATEGORY_LEVEL_FIRST, 10)?;

        let second_categorys = SecondGoodsCategory::belonging_to(&first_categorys)
            .filter(category_level.eq(CATEGORY_LEVEL_SECOND))
            .filter(is_deleted.eq(NOT_DELETE))
            .order(category_rank.desc())
            .load::<SecondGoodsCategory>(conn)?;

        let third_categorys = ThirdGoodsCategory::belonging_to(&second_categorys)
            .filter(category_level.eq(CATEGORY_LEVEL_THIRD))
            .filter(is_deleted.eq(NOT_DELETE))
            .order(category_rank.desc())
            .load::<ThirdGoodsCategory>(conn)?;

        let third_categorys = third_categorys.grouped_by(&second_categorys);

        let second_categorys = second_categorys
            .into_iter()
            .zip(third_categorys)
            .collect::<Vec<_>>();

        let second_categorys = second_categorys.grouped_by(&first_categorys);

        Ok(first_categorys.into_iter().zip(second_categorys).collect())
    }

    pub fn list(conn: &mut PooledConn, level: i8, limit: i64) -> QueryResult<Vec<Self>> {
        tb_newbee_mall_goods_category
            .filter(category_level.eq(level))
            .filter(is_deleted.eq(NOT_DELETE))
            .order(category_rank.desc())
            .limit(limit)
            .load::<Self>(conn)
    }
}