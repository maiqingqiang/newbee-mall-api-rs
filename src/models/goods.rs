use chrono::NaiveDateTime;
use diesel::helper_types::IntoBoxed;
use diesel::mysql::Mysql;

use crate::bootstrap::database::PooledConn;
use crate::models::schema::tb_newbee_mall_goods_info::{dsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::pagination::{Paginator};

use super::pagination::Paginate;

// 上架
pub const ON_SHELF: i8 = 0;
// 下架
pub const OFF_SHELF: i8 = 1;

// 按照发布时间倒序排列
pub const GOOD_ORDER_BY_NEW: &str = "new";
// 按照售价从小到大排列
pub const GOOD_ORDER_BY_PRICE: &str = "price";

#[derive(Debug, Queryable, Serialize)]
pub struct Goods {
    pub goods_id: u64,
    pub goods_name: String,
    pub goods_intro: String,
    pub goods_category_id: i64,
    pub goods_cover_img: String,
    pub goods_carousel: String,
    pub goods_detail_content: String,
    pub original_price: i32,
    pub selling_price: i32,
    pub stock_num: u32,
    pub tag: String,
    pub goods_sell_status: i8,
    pub create_user: i32,
    pub create_time: NaiveDateTime,
    pub update_user: i32,
    pub update_time: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GoodsFilter {
    pub goods_category_id: Option<i64>,
    pub keyword: Option<String>,
    pub order_by: Option<String>,
    pub page_number: Option<i64>,
}

impl Goods {
    // 商品上架状态
    pub const SELL_STATUS_UP: i8 = 0;
    // 商品下架状态
    pub const SELL_STATUS_DOWN: i8 = 1;


    fn filter(filter: &GoodsFilter) -> IntoBoxed<dsl::tb_newbee_mall_goods_info, Mysql> {
        let mut query = dsl::tb_newbee_mall_goods_info.into_boxed();

        if let Some(keyword) = &filter.keyword {
            let keyword = format!("%{}%", keyword);
            query = query.filter(
                dsl::goods_name
                    .like(keyword.clone())
                    .or(dsl::goods_intro.like(keyword)),
            );
        }

        if let Some(category_id) = &filter.goods_category_id {
            query = query.filter(dsl::goods_category_id.eq(category_id));
        }

        query
    }

    pub fn get(conn: &mut PooledConn, filter: &GoodsFilter) -> QueryResult<Paginator<Goods>> {
        Paginate::new(||{
            let mut query = Self::filter(filter);

            if let Some(category_id) = &filter.goods_category_id {
                query = query.filter(dsl::goods_category_id.eq(category_id));
            }
    
            match &filter.order_by.as_deref() {
                Some(GOOD_ORDER_BY_NEW) => query.order(dsl::goods_id.desc()),
                Some(GOOD_ORDER_BY_PRICE) => query.order(dsl::selling_price.asc()),
                // 默认按照库存数量从大到小排列
                _ => query.order(dsl::stock_num.asc()),
            }
        },filter.page_number).load_with_paginator(conn)
    }

    pub fn find(conn: &mut PooledConn, goods_id: u64) -> QueryResult<Self> {
        dsl::tb_newbee_mall_goods_info.find(goods_id).first(conn)
    }

    pub fn get_by_goods_ids(conn: &mut PooledConn, goods_ids: Vec<u64>) -> QueryResult<Vec<Self>> {
        dsl::tb_newbee_mall_goods_info
            .filter(dsl::goods_id.eq_any(&goods_ids))
            .order(dsl::goods_id.eq_any(&goods_ids))
            .load(conn)
    }

    pub fn subtract_stock_num(conn: &mut PooledConn, goods_id: u64, stock_num: u32) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_goods_info)
            .filter(dsl::goods_id.eq(goods_id))
            .filter(dsl::goods_sell_status.eq(Self::SELL_STATUS_UP))
            .filter(dsl::stock_num.gt(stock_num))
            .set(dsl::stock_num.eq(dsl::stock_num - stock_num))
            .execute(conn)
    }

    pub fn add_stock_num(conn: &mut PooledConn, goods_id: u64, stock_num: u32) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_goods_info)
            .filter(dsl::goods_id.eq(goods_id))
            .filter(dsl::goods_sell_status.eq(Self::SELL_STATUS_UP))
            .filter(dsl::stock_num.gt(stock_num))
            .set(dsl::stock_num.eq(dsl::stock_num + stock_num))
            .execute(conn)
    }
}
