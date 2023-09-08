mod utils;

use chrono::{DateTime, Local};
use rand::seq::SliceRandom;
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
    let end = DateTime::parse_from_str("2023 Oct 25 00:00:00.000 +0200", "%Y %b %d %H:%M:%S%.3f %z").unwrap_or_default();

    let duration = end.signed_duration_since(now);

    duration.num_days()
}

const IMAGES: &'static [&'static str] = &[
    "bubu-dance.gif",
    "bubu-dance-cool.gif",
    "bubu-yier-hidden.gif",
    "bubu-yier-dance.gif",
    "bubu-yier-foood.gif",
];

#[wasm_bindgen]
pub fn random_img() -> String {
    let mut rng = rand::thread_rng();
    let choice = IMAGES.choose(&mut rng).unwrap();

    choice.to_string()
}
