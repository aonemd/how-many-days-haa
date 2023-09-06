mod utils;

use chrono::{DateTime, Local};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}!", name));
}

#[wasm_bindgen]
pub fn count_days() -> i64 {
    let now = Local::now();
    let end = DateTime::parse_from_str("2023 Oct 25 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z").unwrap_or_default();

    let duration = end.signed_duration_since(now);

    duration.num_days()
}
