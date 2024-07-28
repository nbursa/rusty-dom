use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{window, Element as WebSysElement};

#[wasm_bindgen]
pub struct RustyElement {
    inner: Rc<RefCell<WebSysElement>>,
}

#[wasm_bindgen]
impl RustyElement {
    #[wasm_bindgen(constructor)]
    pub fn new(tag: &str) -> Result<RustyElement, JsValue> {
        let document = window()
            .ok_or("window not available")?
            .document()
            .ok_or("document not available")?;
        let element = document.create_element(tag)?;
        Ok(RustyElement {
            inner: Rc::new(RefCell::new(element)),
        })
    }

    #[wasm_bindgen]
    pub fn from_id(id: &str) -> Result<RustyElement, JsValue> {
        let document = window()
            .ok_or("window not available")?
            .document()
            .ok_or("document not available")?;
        let element = document.get_element_by_id(id).ok_or("element not found")?;
        Ok(RustyElement {
            inner: Rc::new(RefCell::new(element)),
        })
    }

    #[wasm_bindgen]
    pub fn set_text(&self, text: &str) -> Result<(), JsValue> {
        let element = self.inner.borrow();
        element.set_inner_html(text);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), JsValue> {
        let element = self.inner.borrow();
        element.set_attribute(name, value)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn append_child(&self, child: &RustyElement) -> Result<(), JsValue> {
        let parent = self.inner.borrow();
        let child = child.inner.borrow();
        parent.append_child(&*child)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn render_to_document(&self) -> Result<(), JsValue> {
        let document = window()
            .ok_or("window not available")?
            .document()
            .ok_or("document not available")?;
        let body = document.body().ok_or("body not available")?;
        let app = self.inner.borrow();
        body.append_child(&*app)?;
        Ok(())
    }
}

