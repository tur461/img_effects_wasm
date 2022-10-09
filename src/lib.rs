use base64::decode as b64dec;
use image::load_from_memory;
// log_1 functions logs only 1 value
use web_sys::console::{log_1 as clog1, log_2 as clog2};
use wasm_bindgen::prelude::*;

fn grayscale() {

}

#[wasm_bindgen]
pub fn process_img(b64_content: &str) {
    clog1(&"base64 data recvd".into());
    let res = b64dec(b64_content);
    let mut bytes: Vec<u8> = Vec::new();
    if res.is_ok() {
        bytes = res.unwrap();
        clog1(&"base64 data decoded.".into());
    } else {
        clog1(&"base64 decoding failure!".into());
        return;
    }

    let res = load_from_memory(&bytes[..]);
    
    if res.is_ok() {
        clog1(&"bytes to image done.".into());
    } else {
        clog1(&"failed to convert bytes to images!".into());
    }
}
