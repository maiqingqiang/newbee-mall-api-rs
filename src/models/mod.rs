pub mod admin_user;
pub mod carousel;
pub mod goods;
pub mod goods_category;
pub mod index_config;
pub mod pagination;
pub mod schema;
pub mod user;
pub mod user_token;
pub mod user_address;
pub mod shopping_cart;
pub mod order;
pub mod order_address;
pub mod order_item;

pub use admin_user::*;
pub use goods::*;
pub use goods_category::*;

pub const DELETED: i8 = 1;
pub const NOT_DELETE: i8 = 0;

pub fn pagination(page: Option<i64>, limit: Option<i64>) -> (i64, i64) {
    let mut page = page.unwrap_or(super::constant::DEFAULT_PAGE_NUM);

    if page <= 0 {
        page = super::constant::DEFAULT_PAGE_NUM
    }

    let limit = limit.unwrap_or(super::constant::DEFAULT_PER_PAGE);
    let offset = limit * (page - 1);

    (limit, offset)
}

pub mod functions {
    use diesel::sql_function;

    sql_function! {
        #[aggregate]
        #[sql_name = "last_insert_id"]
        fn last_insert_id_i64() -> Bigint
    }
    sql_function!(fn last_insert_id() -> Bigint);
}

