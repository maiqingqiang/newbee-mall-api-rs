use rand::Rng;

pub fn gen_random_num(length: i32) -> i32 {
    let mut num = 1;

    let random = rand::thread_rng().gen_range(0.1..1.0);

    for _ in 0..length {
        num *= 10
    }

    (random * num as f64) as i32
}

pub fn gen_order_no() -> String {
    gen_random_num(4).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_4_len_random_num_test() {
        print!("{}", gen_random_num(4));
    }

    #[test]
    fn gen_order_no_test() {
        print!("{}", gen_order_no());
    }
}
