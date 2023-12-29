use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::bootstrap::database::PooledConn;
use crate::debug_sql;
use crate::models::pagination::Paginator;
use crate::models::schema::tb_newbee_mall_goods_info::dsl;

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

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::models::schema::tb_newbee_mall_goods_info)]
pub struct UpdateGoods {
    pub goods_id: u64,
    pub goods_name: String,
    pub goods_intro: String,
    pub goods_category_id: i64,
    pub goods_cover_img: String,
    pub goods_detail_content: String,
    pub original_price: i32,
    pub selling_price: i32,
    pub stock_num: u32,
    pub tag: String,
    pub goods_sell_status: i8,
    pub update_user: i32,
    pub update_time: NaiveDateTime,
}

impl UpdateGoods {
    pub fn update(&self, conn: &mut PooledConn) -> QueryResult<usize> {
        let result = diesel::update(dsl::tb_newbee_mall_goods_info)
            .filter(dsl::goods_id.eq(self.goods_id))
            .set(self);

        debug_sql!(&result);

        result.execute(conn)
    }
}

#[derive(Debug)]
pub struct GoodsSearchFilter {
    pub goods_category_id: Option<i64>,
    pub keyword: Option<String>,
    pub order_by: Option<String>,
    pub page_number: Option<i64>,
}

#[derive(Debug)]
pub struct GoodsListFilter {
    pub page_number: Option<i64>,
    pub page_size: Option<i64>,
    pub goods_name: Option<String>,
    pub goods_sell_status: Option<i8>,
}

impl Goods {
    // 商品上架状态
    pub const SELL_STATUS_UP: i8 = 0;
    // 商品下架状态
    pub const SELL_STATUS_DOWN: i8 = 1;

    pub fn get_by_search(
        conn: &mut PooledConn,
        filter: &GoodsSearchFilter,
    ) -> QueryResult<Paginator<Goods>> {
        Paginate::new(
            || {
                let mut query = dsl::tb_newbee_mall_goods_info.into_boxed();

                if let Some(keyword) = &filter.keyword {
                    let keyword = format!("%{keyword}%");
                    query = query.filter(
                        dsl::goods_name
                            .like(keyword.clone())
                            .or(dsl::goods_intro.like(keyword)),
                    );
                }

                if let Some(category_id) = &filter.goods_category_id {
                    query = query.filter(dsl::goods_category_id.eq(category_id));
                }

                if let Some(category_id) = &filter.goods_category_id {
                    query = query.filter(dsl::goods_category_id.eq(category_id));
                }

                match &filter.order_by.as_deref() {
                    Some(GOOD_ORDER_BY_NEW) => query.order(dsl::goods_id.desc()),
                    Some(GOOD_ORDER_BY_PRICE) => query.order(dsl::selling_price.asc()),
                    // 默认按照库存数量从大到小排列
                    _ => query.order(dsl::stock_num.asc()),
                }
            },
            filter.page_number,
        )
        .load_with_paginator(conn)
    }

    pub fn get(conn: &mut PooledConn, filter: &GoodsListFilter) -> QueryResult<Paginator<Goods>> {
        Paginate::new(
            || {
                let mut query = dsl::tb_newbee_mall_goods_info.into_boxed();

                if let Some(keyword) = &filter.goods_name {
                    query = query.filter(dsl::goods_name.like(format!("%{keyword}%")));
                }

                if let Some(goods_sell_status) = &filter.goods_sell_status {
                    query = query.filter(dsl::goods_sell_status.eq(goods_sell_status));
                }

                query
            },
            filter.page_number,
        )
        .per_page(filter.page_size)
        .load_with_paginator(conn)
    }

    pub fn find(conn: &mut PooledConn, goods_id: u64) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_goods_info.find(goods_id);

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn find_by_category_id_name(
        conn: &mut PooledConn,
        category_id: i64,
        name: String,
    ) -> QueryResult<Self> {
        let query = dsl::tb_newbee_mall_goods_info
            .filter(dsl::goods_category_id.eq(category_id))
            .filter(dsl::goods_name.eq(name));

        debug_sql!(&query);

        query.first(conn)
    }

    pub fn get_by_goods_ids(conn: &mut PooledConn, goods_ids: Vec<u64>) -> QueryResult<Vec<Self>> {
        let query = dsl::tb_newbee_mall_goods_info
            .filter(dsl::goods_id.eq_any(&goods_ids))
            .order(dsl::goods_id.eq_any(&goods_ids));

        debug_sql!(&query);

        query.load(conn)
    }

    pub fn subtract_stock_num(
        conn: &mut PooledConn,
        goods_id: u64,
        stock_num: u32,
    ) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_goods_info)
            .filter(dsl::goods_id.eq(goods_id))
            .filter(dsl::goods_sell_status.eq(Self::SELL_STATUS_UP))
            .filter(dsl::stock_num.gt(stock_num))
            .set(dsl::stock_num.eq(dsl::stock_num - stock_num));

        debug_sql!(&query);

        query.execute(conn)
    }

    pub fn add_stock_num(
        conn: &mut PooledConn,
        goods_id: u64,
        stock_num: u32,
    ) -> QueryResult<usize> {
        let query = diesel::update(dsl::tb_newbee_mall_goods_info)
            .filter(dsl::goods_id.eq(goods_id))
            .filter(dsl::goods_sell_status.eq(Self::SELL_STATUS_UP))
            .filter(dsl::stock_num.gt(stock_num))
            .set(dsl::stock_num.eq(dsl::stock_num + stock_num));

        debug_sql!(&query);

        query.execute(conn)
    }
}
