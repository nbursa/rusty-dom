extern crate wasm_bindgen;
extern crate wee_alloc;
extern crate console_error_panic_hook;
extern crate log;
extern crate console_log;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

pub mod element;
pub mod state;
pub mod router;

pub use state::State;
pub use element::{RustyElement, render_to_document};
pub use router::Router;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn initialize() -> Result<(), JsValue> {
    // Initialize logging
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    Ok(())
}

