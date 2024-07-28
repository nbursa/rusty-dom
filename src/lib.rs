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
pub use element::{RustyElement};
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

#[wasm_bindgen]
pub fn create_header(state: &State) -> Result<RustyElement, JsValue> {
    let header = RustyElement::new("h1")?;
    header.set_attribute("class", "header")?;
    header.set_text(&state.get())?;
    Ok(header)
}

#[wasm_bindgen]
pub fn get_element_by_id(id: &str) -> Result<RustyElement, JsValue> {
    RustyElement::from_id(id)
}

