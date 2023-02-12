use crate::app::mall;
use crate::app::mall::OrderListResponse;
use crate::bootstrap::database::PooledConn;
use crate::bootstrap::response::Page;
use crate::bootstrap::result;
use crate::models::goods::Goods;
use crate::models::order::{Filter, NewOrder, Order};
use crate::models::order_address::OrderAddress;
use crate::models::order_item::{NewOrderItem, OrderItem};
use crate::models::shopping_cart::ShoppingCart;
use crate::models::user_address::UserAddress;
use crate::{constant, utils};
use chrono::Local;
use itertools::Itertools;
use std::collections::HashMap;

pub fn save(
    conn: &mut PooledConn,
    user_id: i64,
    address_id: i64,
    cart_item_ids: Vec<i64>,
) -> result::Result<String> {
    let shopping_cart_items =
        super::shopping_cart::get_shopping_cart_items(conn, user_id, cart_item_ids)?;

    if shopping_cart_items.is_empty() {
        return Err("参数异常".into());
    }

    let price_total = shopping_cart_items
        .iter()
        .map(|i| i.selling_price)
        .sum::<i32>();

    if price_total < 1 {
        return Err("价格异常".into());
    }

    let address = UserAddress::find(conn, address_id)?;

    if address.user_id != user_id {
        return Err(constant::REQUEST_FORBIDEN_ERROR.into());
    }

    let goods_ids = shopping_cart_items
        .iter()
        .map(|i| i.goods_id as u64)
        .collect();

    let goods = Goods::get_by_goods_ids(conn, goods_ids)?;

    if goods.is_empty() {
        return Err("购物车数据异常！".into());
    }

    let mut goods_map = HashMap::<i64, Goods>::new();

    for good in goods {
        if good.goods_sell_status != Goods::SELL_STATUS_UP {
            return Err(format!("{}已下架，无法生成订单", good.goods_name).into());
        }

        goods_map.insert(good.goods_id as i64, good);
    }

    let mut cart_item_ids = vec![];

    for item in &shopping_cart_items {
        if goods_map.get(&item.goods_id).is_none() {
            return Err("购物车数据异常！".into());
        }

        if item.goods_count > goods_map.get(&item.goods_id).unwrap().stock_num as i32 {
            return Err("库存不足！".into());
        }

        cart_item_ids.push(item.cart_item_id)
    }

    if ShoppingCart::delete_by_cart_item_ids(conn, cart_item_ids)? > 0 {
        let mut total_price = 0;
        for item in &shopping_cart_items {
            if Goods::subtract_stock_num(conn, item.goods_id as u64, item.goods_count as u32)? < 1 {
                return Err("库存不足！".into());
            }

            total_price += item.goods_count * item.selling_price;
        }

        if total_price < 1 {
            return Err("订单价格异常！".into());
        }

        let order_no = utils::number::gen_order_no();

        let order = Order::create(
            conn,
            NewOrder {
                order_no: order_no.to_string(),
                user_id,
                total_price,
                extra_info: "".to_string(),
            },
        )?;

        let mut order_items = vec![];

        for item in shopping_cart_items {
            order_items.push(NewOrderItem {
                order_id: order.order_id,
                goods_id: item.goods_id,
                goods_name: item.goods_name,
                goods_cover_img: item.goods_cover_img,
                selling_price: item.selling_price,
                goods_count: item.goods_count,
            })
        }

        OrderItem::create_batch(conn, order_items)?;

        OrderAddress::create(
            conn,
            OrderAddress {
                order_id: order.order_id,
                user_name: address.user_name,
                user_phone: address.user_phone,
                province_name: address.province_name,
                city_name: address.city_name,
                region_name: address.region_name,
                detail_address: address.detail_address,
            },
        )?;

        return Ok(order_no);
    }

    Err("购物车数据异常！".into())
}

pub fn list(conn: &mut PooledConn, filter: &Filter) -> result::Result<Page<OrderListResponse>> {
    let orders_with_paginator = Order::get_with_paginator(conn, filter)?;

    let order_ids = orders_with_paginator
        .data
        .iter()
        .map(|i| i.order_id)
        .collect();

    let order_items = OrderItem::get_by_order_ids(conn, order_ids)?;

    let order_items_map = order_items
        .into_iter()
        .into_group_map_by(|item| item.order_id);

    let mut response = vec![];

    for order in orders_with_paginator.data {
        let mut r = OrderListResponse {
            order_id: order.order_id,
            order_no: order.order_no,
            total_price: order.total_price,
            pay_type: order.pay_type,
            order_status: order.order_status,
            order_status_string: String::from(""),
            create_time: order.create_time,
            order_item_vos: vec![],
        };

        if let Some(order_items) = order_items_map.get(&order.order_id) {
            for item in order_items {
                r.order_item_vos.push(mall::OrderItem {
                    goods_id: item.goods_id,
                    goods_count: item.goods_count,
                    goods_name: item.goods_name.clone(),
                    goods_cover_img: item.goods_cover_img.clone(),
                    selling_price: item.selling_price,
                })
            }
        }

        response.push(r)
    }

    Ok(Page::new(
        response,
        orders_with_paginator.total,
        orders_with_paginator.current_page,
        orders_with_paginator.per_page,
    ))
}

pub fn cancel(conn: &mut PooledConn, order_no: String, user_id: i64) -> result::Result<()> {
    let order = Order::find_by_order_no(conn, order_no)?;

    if order.user_id != user_id {
        return Err("无权限！".into());
    }

    if order.order_status == 1
        || order.order_status == -1
        || order.order_status == -2
        || order.order_status == -3
    {
        return Err("订单状态异常！".into());
    }

    Order::update_order_status(conn, vec![order.order_id], -1)?;

    let order_items = OrderItem::get_by_order_id(conn, order.order_id)?;

    for item in order_items {
        Goods::add_stock_num(conn, item.goods_id as u64, item.goods_count as u32)?;
    }

    Ok(())
}

pub fn paid(conn: &mut PooledConn, order_no: String, pay_type: i8) -> result::Result<()> {
    let mut order = Order::find_by_order_no(conn, order_no)?;

    if order.order_status != 0 {
        return Err("订单状态异常！".into());
    }

    order.order_status = 1;
    order.pay_type = pay_type;
    order.pay_status = 1;
    order.pay_time = Some(Local::now().naive_local());
    order.update_time = Local::now().naive_local();

    Order::update(conn, order)?;

    Ok(())
}

pub fn detail(
    conn: &mut PooledConn,
    order_no: String,
    user_id: i64,
) -> result::Result<(Order, Vec<OrderItem>)> {
    let order = Order::find_by_order_no(conn, order_no)?;

    if order.user_id != user_id {
        return Err("禁止该操作！".into());
    }

    let order_items = OrderItem::get_by_order_id(conn, order.order_id)?;

    if order_items.is_empty() {
        return Err("订单项不存在！".into());
    }

    Ok((order, order_items))
}

pub fn finish(conn: &mut PooledConn, order_no: String, user_id: i64) -> result::Result<()> {
    let mut order = Order::find_by_order_no(conn, order_no)?;
    if order.user_id != user_id {
        return Err("禁止该操作！".into());
    }

    if order.order_status != 3 {
        return Err("订单状态异常！".into());
    }

    order.order_status = 4;
    order.update_time = Local::now().naive_local();

    Order::update(conn, order)?;

    Ok(())
}
