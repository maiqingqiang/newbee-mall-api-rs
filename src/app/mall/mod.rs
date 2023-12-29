use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::app::deserialize_option_number_from_string;

pub mod address;
pub mod categories;
pub mod goods;
pub mod index;
pub mod order;
pub mod shop_cart;
pub mod user;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GoodsSearchRequest {
    pub goods_category_id: Option<i64>,
    pub keyword: Option<String>,
    pub order_by: Option<String>,
    pub page_number: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct GoodsSearchResponse {
    pub goods_id: u64,
    pub goods_name: String,
    pub goods_intro: String,
    pub goods_cover_img: String,
    pub selling_price: i32,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct GoodsDetailResponse<'a> {
    pub goods_id: u64,
    pub goods_name: String,
    pub goods_intro: String,
    pub goods_cover_img: String,
    pub selling_price: i32,
    pub goods_detail_content: String,
    pub original_price: i32,
    pub tag: String,
    pub goods_carousel_list: Vec<&'a str>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct GoodsCategoriesResponse {
    pub category_id: i64,
    pub category_level: i8,
    pub category_name: String,
    #[serde(rename = "secondLevelCategoryVOS")]
    pub second_level_category_vos: Vec<GoodsCategories2>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct GoodsCategories2 {
    pub category_id: i64,
    pub parent_id: i64,
    pub category_level: i8,
    pub category_name: String,
    #[serde(rename = "thirdLevelCategoryVOS")]
    pub third_level_category_vos: Vec<GoodsCategories3>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct GoodsCategories3 {
    pub category_id: i64,
    pub category_level: i8,
    pub category_name: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct Carousel {
    pub carousel_url: String,
    pub redirect_url: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct IndexGoods {
    pub goods_id: u64,
    pub goods_name: String,
    pub goods_intro: String,
    pub goods_cover_img: String,
    pub selling_price: i32,
    pub tag: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct IndexResponse {
    pub carousels: Vec<Carousel>,
    pub hot_goodses: Vec<IndexGoods>,
    pub new_goodses: Vec<IndexGoods>,
    pub recommend_goodses: Vec<IndexGoods>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RegisterRequest {
    pub login_name: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LoginRequest {
    pub login_name: String,
    pub password_md5: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserInfoResponse {
    pub nick_name: String,
    pub login_name: String,
    pub introduce_sign: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct EditUserInfoRequest {
    pub nick_name: String,
    pub introduce_sign: String,
    pub password_md5: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct UserAddressListResponse {
    pub address_id: i64,
    pub city_name: String,
    pub default_flag: i8,
    pub detail_address: String,
    pub province_name: String,
    pub region_name: String,
    pub user_id: i64,
    pub user_name: String,
    pub user_phone: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct UserAddressDetailResponse {
    pub address_id: i64,
    pub city_name: String,
    pub default_flag: i8,
    pub detail_address: String,
    pub province_name: String,
    pub region_name: String,
    pub user_id: i64,
    pub user_name: String,
    pub user_phone: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserAddresseSaveRequest {
    pub city_name: String,
    pub default_flag: i8,
    pub detail_address: String,
    pub province_name: String,
    pub region_name: String,
    pub user_name: String,
    pub user_phone: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserAddresseUpdateRequest {
    // 前端给的是字符串
    pub address_id: String,
    pub city_name: String,
    pub default_flag: i8,
    pub detail_address: String,
    pub province_name: String,
    pub region_name: String,
    pub user_name: String,
    pub user_phone: String,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ShoppingCartSaveRequest {
    pub goods_id: i64,
    pub goods_count: i32,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ShoppingCartUpdateRequest {
    pub cart_item_id: i64,
    pub goods_count: i32,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ShoppingCartSettleRequest {
    pub cart_item_ids: String,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ShoppingCartItem {
    pub cart_item_id: i64,
    pub goods_count: i32,
    pub goods_cover_img: String,
    pub goods_id: i64,
    pub goods_name: String,
    pub selling_price: i32,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OrderSaveRequest {
    pub address_id: i64,
    pub cart_item_ids: Vec<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct OrderListRequest {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub status: Option<i8>,
    pub page_number: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct OrderItem {
    pub goods_id: i64,
    pub goods_count: i32,
    pub goods_name: String,
    pub goods_cover_img: String,
    pub selling_price: i32,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct OrderListResponse {
    pub order_id: i64,
    pub order_no: String,
    pub total_price: i32,
    pub pay_type: i8,
    pub order_status: i8,
    pub order_status_string: String,
    pub create_time: NaiveDateTime,
    #[serde(rename = "newBeeMallOrderItemVOS")]
    pub order_item_vos: Vec<OrderItem>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PaySuccessRequest {
    pub order_no: String,
    pub pay_type: i8,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct OrderDetailResponse {
    pub order_no: String,
    pub order_status: i8,
    pub order_status_string: String,
    pub pay_status: i8,
    pub pay_time: Option<NaiveDateTime>,
    pub pay_type: i8,
    pub pay_type_string: String,
    pub total_price: i32,
    pub create_time: NaiveDateTime,
    #[serde(rename = "newBeeMallOrderItemVOS")]
    pub new_bee_mall_order_item_vos: Vec<OrderItem>,
}

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ShoppingCartListRequest {
    pub page_number: Option<i64>,
}
