use wasm_bindgen::prelude::*;
use rusty_dom::{initialize, element::RustyElement, state::State};
use web_sys::window;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    initialize()?;
    main()
}

pub fn main() -> Result<(), JsValue> {
    let state = State::new("Hello, World!".to_string());

    let mut app = RustyElement::new("div");
    let mut header = RustyElement::new("h1");
    header.set_text(&state.get());

    app.append_child(header.clone());

    let rendered = app.render();
    let document = window().unwrap().document().unwrap();
    let app_container = document.get_element_by_id("app").unwrap();
    app_container.set_inner_html(&rendered);

    state.set("Hello, RustyDOM!".to_string());
    header.set_text(&state.get());
    app_container.set_inner_html(&app.render());

    Ok(())
}

