// @generated automatically by Diesel CLI.

diesel::table! {
    tb_newbee_mall_admin_user (admin_user_id) {
        admin_user_id -> Bigint,
        #[max_length = 50]
        login_user_name -> Varchar,
        #[max_length = 50]
        login_password -> Varchar,
        #[max_length = 50]
        nick_name -> Varchar,
        locked -> Nullable<Tinyint>,
    }
}

diesel::table! {
    tb_newbee_mall_admin_user_token (admin_user_id) {
        admin_user_id -> Bigint,
        #[max_length = 32]
        token -> Varchar,
        update_time -> Datetime,
        expire_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_carousel (carousel_id) {
        carousel_id -> Integer,
        #[max_length = 100]
        carousel_url -> Varchar,
        #[max_length = 100]
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
        #[max_length = 50]
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
        #[max_length = 200]
        goods_name -> Varchar,
        #[max_length = 200]
        goods_intro -> Varchar,
        goods_category_id -> Bigint,
        #[max_length = 200]
        goods_cover_img -> Varchar,
        #[max_length = 500]
        goods_carousel -> Varchar,
        goods_detail_content -> Text,
        original_price -> Integer,
        selling_price -> Integer,
        stock_num -> Unsigned<Integer>,
        #[max_length = 20]
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
        #[max_length = 50]
        config_name -> Varchar,
        config_type -> Tinyint,
        goods_id -> Bigint,
        #[max_length = 100]
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
        #[max_length = 20]
        order_no -> Varchar,
        user_id -> Bigint,
        total_price -> Integer,
        pay_status -> Tinyint,
        pay_type -> Tinyint,
        pay_time -> Nullable<Datetime>,
        order_status -> Tinyint,
        #[max_length = 100]
        extra_info -> Varchar,
        is_deleted -> Tinyint,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_order_address (order_id) {
        order_id -> Bigint,
        #[max_length = 30]
        user_name -> Varchar,
        #[max_length = 11]
        user_phone -> Varchar,
        #[max_length = 32]
        province_name -> Varchar,
        #[max_length = 32]
        city_name -> Varchar,
        #[max_length = 32]
        region_name -> Varchar,
        #[max_length = 64]
        detail_address -> Varchar,
    }
}

diesel::table! {
    tb_newbee_mall_order_item (order_item_id) {
        order_item_id -> Bigint,
        order_id -> Bigint,
        goods_id -> Bigint,
        #[max_length = 200]
        goods_name -> Varchar,
        #[max_length = 200]
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
        #[max_length = 50]
        nick_name -> Varchar,
        #[max_length = 11]
        login_name -> Varchar,
        #[max_length = 32]
        password_md5 -> Varchar,
        #[max_length = 100]
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
        #[max_length = 30]
        user_name -> Varchar,
        #[max_length = 11]
        user_phone -> Varchar,
        default_flag -> Tinyint,
        #[max_length = 32]
        province_name -> Varchar,
        #[max_length = 32]
        city_name -> Varchar,
        #[max_length = 32]
        region_name -> Varchar,
        #[max_length = 64]
        detail_address -> Varchar,
        is_deleted -> Tinyint,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

diesel::table! {
    tb_newbee_mall_user_token (user_id) {
        user_id -> Bigint,
        #[max_length = 32]
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
