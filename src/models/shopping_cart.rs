use crate::bootstrap::database::PooledConn;
use crate::models::pagination::{Paginate, Paginator};
use crate::models::schema::tb_newbee_mall_shopping_cart_item::dsl;
use crate::models::{schema, DELETED, NOT_DELETE};
use chrono::NaiveDateTime;
use diesel::dsl::IntoBoxed;
use diesel::mysql::Mysql;
use diesel::prelude::*;

#[derive(Debug, Queryable, AsChangeset)]
#[diesel(table_name = schema::tb_newbee_mall_shopping_cart_item)]
pub struct ShoppingCart {
    pub cart_item_id: i64,
    pub user_id: i64,
    pub goods_id: i64,
    pub goods_count: i32,
    pub is_deleted: i8,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::tb_newbee_mall_shopping_cart_item)]
pub struct NewShoppingCart {
    pub user_id: i64,
    pub goods_id: i64,
    pub goods_count: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = schema::tb_newbee_mall_shopping_cart_item)]
pub struct UpdateShoppingCart {
    pub goods_count: i32,
    pub update_time: NaiveDateTime,
}

pub struct ShoppingCartItem {
    pub cart_item_id: i64,
    pub goods_id: i64,
    pub goods_count: i32,
    pub goods_name: String,
    pub goods_cover_img: String,
    pub selling_price: i32,
}

impl ShoppingCart {
    // 购物车中商品的最大数量(可根据自身需求修改)
    pub const SHOPPING_CART_TOTAL_NUMBER: i64 = 20;
    // 购物车分页的默认条数(每页5条)
    pub const SHOPPING_CART_PAGE_LIMIT: i64 = 5;
    // 购物车中单个商品的最大购买数量(可根据自身需求修改)
    pub const SHOPPING_CART_ITEM_LIMIT_NUMBER: i32 = 5;

    fn filter(user_id: i64) -> IntoBoxed<'static, dsl::tb_newbee_mall_shopping_cart_item, Mysql> {
        dsl::tb_newbee_mall_shopping_cart_item
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .into_boxed()
    }

    pub fn get(conn: &mut PooledConn, user_id: i64) -> QueryResult<Vec<Self>> {
        Self::filter(user_id)
            .limit(Self::SHOPPING_CART_TOTAL_NUMBER)
            .load::<Self>(conn)
    }

    pub fn get_with_page(
        conn: &mut PooledConn,
        user_id: i64,
        page: Option<i64>,
    ) -> QueryResult<Paginator<Self>> {
        Paginate::new(|| Self::filter(user_id), page).load_with_paginator(conn)
    }

    pub fn create(conn: &mut PooledConn, shopping_cart: NewShoppingCart) -> QueryResult<usize> {
        diesel::insert_into(dsl::tb_newbee_mall_shopping_cart_item)
            .values(shopping_cart)
            .execute(conn)
    }

    pub fn update(conn: &mut PooledConn, shopping_cart: Self) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_shopping_cart_item.find(shopping_cart.cart_item_id))
            .set(shopping_cart)
            .execute(conn)
    }

    pub fn delete(conn: &mut PooledConn, cart_item_id: i64) -> QueryResult<usize> {
        Self::delete_by_cart_item_ids(conn, vec![cart_item_id])
    }

    pub fn delete_by_cart_item_ids(
        conn: &mut PooledConn,
        cart_item_ids: Vec<i64>,
    ) -> QueryResult<usize> {
        diesel::update(dsl::tb_newbee_mall_shopping_cart_item)
            .filter(dsl::cart_item_id.eq_any(cart_item_ids))
            .set(dsl::is_deleted.eq(DELETED))
            .execute(conn)
    }

    pub fn get_by_cart_item_ids(
        conn: &mut PooledConn,
        user_id: i64,
        cart_item_ids: &Vec<i64>,
    ) -> QueryResult<Vec<Self>> {
        Self::filter(user_id)
            .filter(dsl::cart_item_id.eq_any(cart_item_ids))
            .load::<Self>(conn)
    }
    pub fn find(conn: &mut PooledConn, cart_item_id: i64) -> QueryResult<Self> {
        dsl::tb_newbee_mall_shopping_cart_item
            .filter(dsl::is_deleted.eq(NOT_DELETE))
            .find(cart_item_id)
            .first(conn)
    }

    pub fn find_by_user_id_goods_id(
        conn: &mut PooledConn,
        user_id: i64,
        goods_id: i64,
    ) -> QueryResult<Self> {
        Self::filter(user_id)
            .filter(dsl::goods_id.eq(goods_id))
            .first(conn)
    }

    pub fn count(
        conn: &mut PooledConn,
        user_id: i64,
    ) -> QueryResult<i64> {
        Self::filter(user_id)
            .count()
            .first(conn)
    }
}
