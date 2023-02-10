pub mod admin_user;
pub mod admin_user_token;
pub mod carousel;
pub mod goods;
pub mod goods_category;
pub mod index_config;
pub mod order;
pub mod order_address;
pub mod order_item;
pub mod pagination;
pub mod schema;
pub mod shopping_cart;
pub mod user;
pub mod user_address;
pub mod user_token;

pub use admin_user::*;
pub use goods::*;
pub use goods_category::*;

pub const DELETED: i8 = 1;
pub const NOT_DELETE: i8 = 0;

pub const LOCKED: i8 = 1;
pub const NOT_LOCK: i8 = 0;

pub mod functions {
    use diesel::sql_function;

    sql_function! {
        #[aggregate]
        #[sql_name = "last_insert_id"]
        fn last_insert_id_i64() -> Bigint
    }
    sql_function!(fn last_insert_id() -> Bigint);
}
