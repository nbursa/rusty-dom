use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{window, Element as WebSysElement, HtmlElement, EventTarget};

#[wasm_bindgen]
pub struct RustyElement {
    inner: Rc<RefCell<WebSysElement>>,
}

#[wasm_bindgen]
impl RustyElement {
    #[wasm_bindgen(constructor)]
    pub fn new(tag: &str) -> Result<RustyElement, JsValue> {
        let document = window().unwrap().document().unwrap();
        let element = document.create_element(tag)?;
        Ok(RustyElement {
            inner: Rc::new(RefCell::new(element)),
        })
    }

    pub fn from_id(id: &str) -> Result<RustyElement, JsValue> {
        let document = window().unwrap().document().unwrap();
        let element = document.get_element_by_id(id).ok_or_else(|| JsValue::from_str("Element not found"))?;
        Ok(RustyElement {
            inner: Rc::new(RefCell::new(element)),
        })
    }

    pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), JsValue> {
        self.inner.borrow().set_attribute(name, value)
    }

    pub fn set_text(&self, text: &str) -> Result<(), JsValue> {
        self.inner.borrow().set_text_content(Some(text));
        Ok(())
    }

    pub fn add_event_listener(&self, event: &str, callback: &Closure<dyn FnMut()>) -> Result<(), JsValue> {
        let element: EventTarget = self.inner.borrow().clone().dyn_into()?;
        element.add_event_listener_with_callback(event, callback.as_ref().unchecked_ref())?;
        callback.forget();  // Call forget to prevent memory leaks
        Ok(())
    }

    pub fn set_style(&self, property: &str, value: &str) -> Result<(), JsValue> {
        let html_element: &HtmlElement = self.inner.borrow().dyn_ref().expect("Element is not an HtmlElement");
        html_element.style().set_property(property, value)
    }
}

