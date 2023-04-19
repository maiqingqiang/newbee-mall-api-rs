use md5::Md5;
use md5::Digest;

pub mod number;
pub mod token;

pub fn md5(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn md5_string(s: String) -> String {
    md5(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(md5("hello world"), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    #[test]
    fn test_md5_string() {
        assert_eq!(md5_string(String::from("hello world")), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }
}