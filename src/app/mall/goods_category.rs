use actix_web::{get, web};

use crate::app::mall::{GoodsCategories2, GoodsCategories3, GoodsCategoriesResponse};
use crate::bootstrap::database::DatabasePool;
use crate::bootstrap::response::Response;
use crate::bootstrap::result;
use crate::services;

// 新蜂商城分类页面接口
#[get("/categories")]
pub async fn categories(pool: web::Data<DatabasePool>) -> result::Response {
    let pool = &mut pool.get()?;

    let categories = services::goods_category::collect(pool)?;

    let mut response: Vec<GoodsCategoriesResponse> = vec![];

    for (category, sub_categories2) in categories {
        let mut response2: Vec<GoodsCategories2> = vec![];

        for (category2, sub_categories3) in sub_categories2 {
            let mut response3: Vec<GoodsCategories3> = vec![];

            for category3 in sub_categories3 {
                response3.push(GoodsCategories3 {
                    category_id: category3.category_id,
                    category_level: category3.category_level,
                    category_name: category3.category_name,
                })
            }

            response2.push(GoodsCategories2 {
                category_id: category2.category_id,
                category_level: category2.category_level,
                parent_id: category2.parent_id,
                category_name: category2.category_name,
                third_level_category_vos: response3,
            })
        }

        response.push(GoodsCategoriesResponse {
            category_id: category.category_id,
            category_level: category.category_level,
            category_name: category.category_name,
            second_level_category_vos: response2,
        })
    }

    Response::success(response)
}
