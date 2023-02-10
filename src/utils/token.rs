use crate::utils::md5_string;
use chrono::Local;
use rand::Rng;

pub fn generate_token(id: i64) -> String {
    let s = format!(
        "{}{}{}",
        Local::now().timestamp_millis(),
        id,
        rand::thread_rng().gen_range(1000..10000)
    );

    md5_string(s)
}
