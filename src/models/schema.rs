// @generated automatically by Diesel CLI.

diesel::table! {
    tb_newbee_mall_admin_user (admin_user_id) {
        admin_user_id -> Bigint,
        login_user_name -> Varchar,
        login_password -> Varchar,
        nick_name -> Varchar,
        locked -> Nullable<Tinyint>,
    }
}

diesel::table! {
    tb_newbee_mall_admin_user_token (admin_user_id) {
        admin_user_id -> Bigint,
        token -> Varchar,
        update_time -> Datetime,
        expire_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_carousel (carousel_id) {
        carousel_id -> Integer,
        carousel_url -> Varchar,
        redirect_url -> Varchar,
        carousel_rank -> Integer,
        is_deleted -> Tinyint,
        create_time -> Datetime,
        create_user -> Integer,
        update_time -> Datetime,
        update_user -> Integer,
    }
}

diesel::table! {
    tb_newbee_mall_goods_category (category_id) {
        category_id -> Bigint,
        category_level -> Tinyint,
        parent_id -> Bigint,
        category_name -> Varchar,
        category_rank -> Integer,
        is_deleted -> Tinyint,
        create_time -> Datetime,
        create_user -> Integer,
        update_time -> Datetime,
        update_user -> Nullable<Integer>,
    }
}

diesel::table! {
    tb_newbee_mall_goods_info (goods_id) {
        goods_id -> Unsigned<Bigint>,
        goods_name -> Varchar,
        goods_intro -> Varchar,
        goods_category_id -> Bigint,
        goods_cover_img -> Varchar,
        goods_carousel -> Varchar,
        goods_detail_content -> Text,
        original_price -> Integer,
        selling_price -> Integer,
        stock_num -> Unsigned<Integer>,
        tag -> Varchar,
        goods_sell_status -> Tinyint,
        create_user -> Integer,
        create_time -> Datetime,
        update_user -> Integer,
        update_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_index_config (config_id) {
        config_id -> Bigint,
        config_name -> Varchar,
        config_type -> Tinyint,
        goods_id -> Bigint,
        redirect_url -> Varchar,
        config_rank -> Integer,
        is_deleted -> Tinyint,
        create_time -> Datetime,
        create_user -> Integer,
        update_time -> Datetime,
        update_user -> Nullable<Integer>,
    }
}

diesel::table! {
    tb_newbee_mall_order (order_id) {
        order_id -> Bigint,
        order_no -> Varchar,
        user_id -> Bigint,
        total_price -> Integer,
        pay_status -> Tinyint,
        pay_type -> Tinyint,
        pay_time -> Nullable<Datetime>,
        order_status -> Tinyint,
        extra_info -> Varchar,
        is_deleted -> Tinyint,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_order_address (order_id) {
        order_id -> Bigint,
        user_name -> Varchar,
        user_phone -> Varchar,
        province_name -> Varchar,
        city_name -> Varchar,
        region_name -> Varchar,
        detail_address -> Varchar,
    }
}

diesel::table! {
    tb_newbee_mall_order_item (order_item_id) {
        order_item_id -> Bigint,
        order_id -> Bigint,
        goods_id -> Bigint,
        goods_name -> Varchar,
        goods_cover_img -> Varchar,
        selling_price -> Integer,
        goods_count -> Integer,
        create_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_shopping_cart_item (cart_item_id) {
        cart_item_id -> Bigint,
        user_id -> Bigint,
        goods_id -> Bigint,
        goods_count -> Integer,
        is_deleted -> Tinyint,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_user (user_id) {
        user_id -> Bigint,
        nick_name -> Varchar,
        login_name -> Varchar,
        password_md5 -> Varchar,
        introduce_sign -> Varchar,
        is_deleted -> Tinyint,
        locked_flag -> Tinyint,
        create_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_user_address (address_id) {
        address_id -> Bigint,
        user_id -> Bigint,
        user_name -> Varchar,
        user_phone -> Varchar,
        default_flag -> Tinyint,
        province_name -> Varchar,
        city_name -> Varchar,
        region_name -> Varchar,
        detail_address -> Varchar,
        is_deleted -> Tinyint,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_user_token (user_id) {
        user_id -> Bigint,
        token -> Varchar,
        update_time -> Datetime,
        expire_time -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tb_newbee_mall_admin_user,
    tb_newbee_mall_admin_user_token,
    tb_newbee_mall_carousel,
    tb_newbee_mall_goods_category,
    tb_newbee_mall_goods_info,
    tb_newbee_mall_index_config,
    tb_newbee_mall_order,
    tb_newbee_mall_order_address,
    tb_newbee_mall_order_item,
    tb_newbee_mall_shopping_cart_item,
    tb_newbee_mall_user,
    tb_newbee_mall_user_address,
    tb_newbee_mall_user_token,
);
