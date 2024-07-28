extern crate wasm_bindgen;
extern crate wee_alloc;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

pub mod element;
pub mod state;
mod router;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn initialize() -> Result<(), JsValue> {
    // Initialize logging
    console_error_panic_hook::set_once();

    Ok(())
}

