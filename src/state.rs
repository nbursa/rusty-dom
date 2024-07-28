use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct State {
    value: Rc<RefCell<String>>,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: &str) -> State {
        State {
            value: Rc::new(RefCell::new(initial.to_string())),
        }
    }

    pub fn set(&self, value: &str) {
        *self.value.borrow_mut() = value.to_string();
    }

    pub fn get(&self) -> String {
        self.value.borrow().clone()
    }
}

