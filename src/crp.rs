
use base64::prelude::*;

pub fn encode_to_base64(str: &mut String) {
    *str = BASE64_STANDARD.encode(str.as_bytes())
}

pub fn decode_from_base64(str: &mut String) {
    *str = String::from_utf8(BASE64_STANDARD.decode(str.as_bytes()).unwrap()).unwrap()
}
