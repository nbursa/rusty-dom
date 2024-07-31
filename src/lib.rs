extern crate console_error_panic_hook;
extern crate console_log;
extern crate log;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub mod element;
pub mod router;
pub mod state;

pub use element::RustyElement;
pub use router::Router;
pub use state::State;

#[wasm_bindgen]
pub fn initialize() -> Result<(), JsValue> {
    // Initialize logging
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    Ok(())
}
