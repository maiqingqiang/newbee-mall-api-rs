use crypto::digest::Digest;
use crypto::md5::Md5;

pub mod number;
pub mod token;

pub fn md5(s: &str) -> String {
    let mut md5 = Md5::new();
    md5.input_str(s);
    return md5.result_str();
}

pub fn md5_string(s: String) -> String {
    let mut md5 = Md5::new();
    md5.input_str(s.as_str());
    return md5.result_str();
}
