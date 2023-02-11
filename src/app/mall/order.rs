use crate::app::mall::{
    OrderDetailResponse, OrderItem, OrderListRequest, OrderSaveRequest, PaySuccessRequest,
};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::middleware::authentication::MallIdentity;
use crate::models::order;
use crate::{get_order_status_str, services};
use actix_web::{get, post, put, web};

// 生成订单接口
#[post("/saveOrder")]
pub async fn save(
    pool: web::Data<DatabasePool>,
    web::Json(json): web::Json<OrderSaveRequest>,
    identity: MallIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let order_no = services::order::save(
        conn,
        identity.user.user_id,
        json.address_id,
        json.cart_item_ids,
    )?;

    Response::success(order_no)
}

// 订单详情接口
#[get("/{orderNo}")]
pub async fn detail(
    pool: web::Data<DatabasePool>,
    order_no: web::Path<String>,
    identity: MallIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    let (order, order_items) =
        services::order::detail(conn, order_no.into_inner(), identity.user.user_id)?;

    let mut new_bee_mall_order_item_vos = vec![];

    for order_item in order_items {
        new_bee_mall_order_item_vos.push(OrderItem {
            goods_id: order_item.goods_id,
            goods_count: order_item.goods_count,
            goods_name: order_item.goods_name,
            goods_cover_img: order_item.goods_cover_img,
            selling_price: order_item.selling_price,
        })
    }

    Response::success(OrderDetailResponse {
        order_no: order.order_no,
        order_status: order.order_status,
        order_status_string: get_order_status_str(order.order_status).to_string(),
        pay_status: order.pay_status,
        pay_time: order.pay_time,
        pay_type: order.pay_type,
        pay_type_string: get_order_status_str(order.pay_type).to_string(),
        total_price: order.total_price,
        create_time: order.create_time,
        new_bee_mall_order_item_vos,
    })
}

// 订单列表接口
#[get("")]
pub async fn list(
    pool: web::Data<DatabasePool>,
    web::Query(query): web::Query<OrderListRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    let page_number = query.page_number.unwrap_or(0);

    let response = services::order::list(
        conn,
        &order::Filter {
            status: query.status,
            page_number: page_number.into(),
        },
    )?;

    Response::success(response)
}

// 订单取消接口
#[put("/{orderNo}/cancel")]
pub async fn cancel(
    pool: web::Data<DatabasePool>,
    order_no: web::Path<String>,
    identity: MallIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::order::cancel(conn, order_no.into_inner(), identity.user.user_id)?;

    Response::success(())
}

// 确认收货接口
#[get("/{orderNo}/finish")]
pub async fn finish(
    pool: web::Data<DatabasePool>,
    order_no: web::Path<String>,
    identity: MallIdentity,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::order::finish(conn, order_no.into_inner(), identity.user.user_id)?;

    Response::success(())
}

// 模拟支付成功回调的接口
#[get("/paySuccess")]
pub async fn pay_success(
    pool: web::Data<DatabasePool>,
    web::Query(query): web::Query<PaySuccessRequest>,
) -> result::Response {
    let conn = &mut pool.get()?;

    services::order::paid(conn, query.order_no, query.pay_type)?;

    Response::success(())
}
