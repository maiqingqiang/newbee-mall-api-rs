use std::collections::HashMap;

use chrono::Local;
use diesel::NotFound;

use crate::app::mall::ShoppingCartItem;
use crate::bootstrap::database::PooledConn;
use crate::bootstrap::error::ApplicationError;
use crate::bootstrap::result;
use crate::constant;
use crate::constant::{SHOPPING_CART_ITEM_LIMIT_NUMBER, SHOPPING_CART_ITEM_TOTAL_NUMBER};
use crate::models::goods::Goods;
use crate::models::pagination::Paginator;
use crate::models::shopping_cart::{NewShoppingCart, ShoppingCart};

pub fn list(conn: &mut PooledConn, user_id: i64) -> result::Result<Vec<ShoppingCartItem>> {
    let shopping_carts = ShoppingCart::get(conn, user_id)?;
    to_shopping_cart_items(conn, shopping_carts)
}

pub fn list_with_page(
    conn: &mut PooledConn,
    user_id: i64,
    page: Option<i64>,
) -> result::Result<Paginator<ShoppingCart>> {
    let shopping_carts = ShoppingCart::get_with_page(conn, user_id, page)?;
    Ok(shopping_carts)
}

pub fn save(conn: &mut PooledConn, user_id: i64, cart: NewShoppingCart) -> result::Result<usize> {
    if ShoppingCart::find_by_user_id_goods_id(conn, user_id, cart.goods_id).is_ok() {
        return Err("已存在！无需重复添加！".into());
    }

    Goods::find(conn, cart.goods_id as u64)?;

    match Goods::find(conn, cart.goods_id as u64) {
        Ok(_) => {}
        Err(e) => {
            if e == NotFound {
                return Err("商品不存在！".into());
            }
            return Err(e.into());
        }
    }

    if cart.goods_count < 1 {
        return Err("商品数量不能小于 1 ！".into());
    }

    if cart.goods_count > SHOPPING_CART_ITEM_LIMIT_NUMBER {
        return Err("超出单个商品的最大购买数量！".into());
    }

    let total = ShoppingCart::count(conn, user_id)?;

    if (total as i32) > SHOPPING_CART_ITEM_TOTAL_NUMBER {
        return Err("超出购物车最大容量！".into());
    }

    Ok(ShoppingCart::create(conn, cart)?)
}

pub fn update(
    conn: &mut PooledConn,
    user_id: i64,
    cart_item_id: i64,
    goods_count: i32,
) -> result::Result<usize> {
    match ShoppingCart::find(conn, cart_item_id) {
        Ok(mut shopping_cart) => {
            if shopping_cart.user_id != user_id {
                return Err(constant::NO_PERMISSION_ERROR.into());
            }

            if shopping_cart.goods_count > ShoppingCart::SHOPPING_CART_ITEM_LIMIT_NUMBER {
                return Err("超出单个商品的最大购买数量！".into());
            }

            if shopping_cart.goods_count == goods_count {
                return Ok(0);
            }

            shopping_cart.goods_count = goods_count;
            shopping_cart.update_time = Local::now().naive_local();

            Ok(ShoppingCart::update(conn, shopping_cart)?)
        }
        Err(_) => Err(constant::DATA_NOT_EXIST.into()),
    }
}

pub fn delete(conn: &mut PooledConn, user_id: i64, cart_item_id: i64) -> result::Result<usize> {
    match ShoppingCart::find(conn, cart_item_id) {
        Ok(shopping_cart) => {
            if shopping_cart.user_id != user_id {
                return Err(constant::NO_PERMISSION_ERROR.into());
            }

            Ok(ShoppingCart::delete(conn, shopping_cart.cart_item_id)?)
        }
        Err(_) => Ok(0),
    }
}

pub fn settle(
    conn: &mut PooledConn,
    user_id: i64,
    cart_item_ids: Vec<i64>,
) -> result::Result<Vec<ShoppingCartItem>> {
    let shopping_cart_items = get_shopping_cart_items(conn, user_id, cart_item_ids)?;

    let price_total = shopping_cart_items
        .iter()
        .map(|i| i.selling_price)
        .sum::<i32>();

    if price_total < 1 {
        return Err("价格异常".into());
    }

    Ok(shopping_cart_items)
}

pub(crate) fn get_shopping_cart_items(
    conn: &mut PooledConn,
    user_id: i64,
    cart_item_ids: Vec<i64>,
) -> result::Result<Vec<ShoppingCartItem>> {
    if cart_item_ids.is_empty() {
        return Err("购物项不能为空".into());
    }

    let shopping_carts = ShoppingCart::get_by_cart_item_ids(conn, user_id, &cart_item_ids)?;

    if shopping_carts.is_empty() {
        return Err("购物项不能为空".into());
    }

    if shopping_carts.len() != cart_item_ids.len() {
        return Err("参数异常".into());
    }

    to_shopping_cart_items(conn, shopping_carts)
}

pub fn to_shopping_cart_items(
    conn: &mut PooledConn,
    shopping_carts: Vec<ShoppingCart>,
) -> Result<Vec<ShoppingCartItem>, ApplicationError> {
    if shopping_carts.is_empty() {
        return Ok(vec![]);
    }

    let goods_ids: Vec<u64> = shopping_carts.iter().map(|c| c.goods_id as u64).collect();

    let goods = Goods::get_by_goods_ids(conn, goods_ids)?;

    let goods_map = goods
        .iter()
        .map(|good| (good.goods_id, good))
        .collect::<HashMap<u64, &Goods>>();

    let mut shopping_cart_items: Vec<ShoppingCartItem> = vec![];

    for shopping_cart in shopping_carts {
        let good = goods_map.get(&(shopping_cart.goods_id as u64)).unwrap();

        shopping_cart_items.push(ShoppingCartItem {
            cart_item_id: shopping_cart.cart_item_id,
            goods_count: shopping_cart.goods_count,
            goods_cover_img: good.goods_cover_img.to_string(),
            goods_id: shopping_cart.goods_id,
            goods_name: good.goods_name.to_string(),
            selling_price: good.selling_price,
        });
    }

    if shopping_cart_items.is_empty() {
        return Err("参数异常".into());
    }

    Ok(shopping_cart_items)
}
