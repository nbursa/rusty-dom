use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct State<T> {
    value: Rc<RefCell<T>>,
}

impl<T> State<T> {
    pub fn new(initial: T) -> State<T> {
        State {
            value: Rc::new(RefCell::new(initial)),
        }
    }

    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
    }

    pub fn get(&self) -> T where T: Clone {
        self.value.borrow().clone()
    }
}

