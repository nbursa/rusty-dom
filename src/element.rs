use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use web_sys::{Document, Element as WebSysElement, Window};

#[wasm_bindgen]
pub struct RustyElement {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<RustyElement>,
    text: Option<String>,
}

#[wasm_bindgen]
impl RustyElement {
    #[wasm_bindgen(constructor)]
    pub fn new(tag: &str) -> RustyElement {
        let element = RustyElement {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            children: vec![],
            text: None,
        };
        web_sys::console::log_1(&format!("Created RustyElement at: {:?}", &element as *const _).into());
        element
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn append_child(&mut self, child: RustyElement) {
        self.children.push(child);
    }

    pub fn set_text(&mut self, text: &str) {
        web_sys::console::log_1(&format!("Setting text to: {} for RustyElement at: {:?}", text, self as *const _).into());
        self.text = Some(text.to_string());
    }

    pub fn render(&self) -> String {
        let mut html = format!("<{}", self.tag);
        for (key, value) in &self.attributes {
            html.push_str(&format!(" {}=\"{}\"", key, value));
        }
        html.push_str(">");
        if let Some(ref text) = self.text {
            html.push_str(text);
        }
        for child in &self.children {
            html.push_str(&child.render());
        }
        html.push_str(&format!("</{}>", self.tag));
        html
    }
}

#[wasm_bindgen]
pub fn render_to_document(element: &RustyElement) -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();
    let body: WebSysElement = document.body().unwrap().into();

    let rendered_html = element.render();
    body.set_inner_html(&rendered_html);

    Ok(())
}

