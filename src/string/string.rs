use std::str;

pub fn u8_to_str(buf: &[u8]) -> &str {
    return match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}
