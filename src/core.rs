use crate::input::InnerMind;
use crate::query;
use std::cell::RefCell;
thread_local! {
    pub static INSTANCE: RefCell<InnerMind> =
        RefCell::new(InnerMind::new());
}
pub struct MindX;
impl MindX {
    pub fn input(text: &str) -> String {
        INSTANCE.with(|m| m.borrow_mut().process(text))
    }
    pub fn feed(text: &str) {
        INSTANCE.with(|m| {
            m.borrow_mut().perceive(text);
        });
    }
    pub fn query(text: &str) -> String {
        INSTANCE.with(|m| query::associate_query(&m.borrow(), text))
    }
}
