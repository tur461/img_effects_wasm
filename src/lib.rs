// log_1 functions logs only 1 value
use web_sys::console::{log_1 as clog1, log_2 as clog2};
use wasm_bindgen::prelude::*;

fn grayscale() {

}

#[wasm_bindgen]
pub fn process_img(b64_content: &str) {
    clog1(&"hello from rust side!!".into());
    clog2(
        &"this is the b64 data passed to me:".into(), 
        &b64_content.into()
    );
}
