use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
#[derive(Clone)]  // Add Clone trait
pub struct Element {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<Element>,
    text: Option<String>,
}

#[wasm_bindgen]
impl Element {
    #[wasm_bindgen(constructor)]
    pub fn new(tag: &str) -> Element {
        Element {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            children: vec![],
            text: None,
        }
    }

    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn append_child(&mut self, child: Element) {
        self.children.push(child);
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = Some(text.to_string());
    }

    pub fn render(&self) -> String {
        let mut html = format!("<{}", self.tag);
        for (key, value) in &self.attributes {
            html.push_str(&format!(" {}=\"{}\"", key, value));
        }
        html.push_str(">");
        if let Some(text) = &self.text {
            html.push_str(text);
        }
        for child in &self.children {
            html.push_str(&child.render());
        }
        html.push_str(&format!("</{}>", self.tag));
        html
    }
}

