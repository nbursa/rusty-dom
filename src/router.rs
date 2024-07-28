use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use js_sys::Function;

#[wasm_bindgen]
pub struct Router {
    routes: HashMap<String, JsValue>,
}

#[wasm_bindgen]
impl Router {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path: String, handler: JsValue) {
        self.routes.insert(path, handler);
    }

    pub fn navigate(&self, path: &str) {
        if let Some(handler) = self.routes.get(path) {
            let handler: &Function = handler.dyn_ref().unwrap();
            handler.call0(&JsValue::NULL).unwrap();
        }
    }
}

