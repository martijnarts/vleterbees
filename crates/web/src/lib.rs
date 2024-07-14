use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

use app::*;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
