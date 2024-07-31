use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element as WebSysElement, Event, EventTarget, HtmlElement};

#[wasm_bindgen]
pub struct RustyElement {
    inner: Rc<RefCell<WebSysElement>>,
    listeners: Rc<RefCell<Vec<Closure<dyn FnMut(Event)>>>>,
}

#[wasm_bindgen]
impl RustyElement {
    #[wasm_bindgen(constructor)]
    pub fn new(tag: &str) -> Result<RustyElement, JsValue> {
        let document = window().unwrap().document().unwrap();
        let element = document.create_element(tag)?;
        Ok(RustyElement {
            inner: Rc::new(RefCell::new(element)),
            listeners: Rc::new(RefCell::new(Vec::new())),
        })
    }

    #[wasm_bindgen]
    pub fn from_id(id: &str) -> Result<RustyElement, JsValue> {
        let document = window().unwrap().document().unwrap();
        let element = document
            .get_element_by_id(id)
            .ok_or_else(|| JsValue::from_str("Element not found"))?;
        Ok(RustyElement {
            inner: Rc::new(RefCell::new(element)),
            listeners: Rc::new(RefCell::new(Vec::new())),
        })
    }

    #[wasm_bindgen]
    pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), JsValue> {
        self.inner.borrow().set_attribute(name, value)
    }

    #[wasm_bindgen]
    pub fn set_text(&self, text: &str) -> Result<(), JsValue> {
        self.inner.borrow().set_text_content(Some(text));
        Ok(())
    }

    #[wasm_bindgen]
    pub fn add_event_listener(
        &self,
        event: &str,
        callback: js_sys::Function,
    ) -> Result<(), JsValue> {
        let element: EventTarget = self.inner.borrow().clone().dyn_into()?;
        let closure = Closure::wrap(Box::new(move |event: Event| {
            callback.call1(&JsValue::NULL, &event).unwrap();
        }) as Box<dyn FnMut(Event)>);
        element.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())?;
        self.listeners.borrow_mut().push(closure);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn set_style(&self, property: &str, value: &str) -> Result<(), JsValue> {
        let element = self.inner.borrow();
        let html_element: &web_sys::HtmlElement = element
            .dyn_ref()
            .ok_or_else(|| JsValue::from_str("Element is not an HtmlElement"))?;
        html_element.style().set_property(property, value)?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn append_child(&self, child: &RustyElement) -> Result<(), JsValue> {
        self.inner.borrow().append_child(&child.inner.borrow())?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn click(&self) -> Result<(), JsValue> {
        let binding = self.inner.borrow();
        let html_element: &HtmlElement = binding.dyn_ref().expect("Element is not an HtmlElement");
        let event = Event::new("click")?;
        html_element.dispatch_event(&event)?;
        Ok(())
    }
}
