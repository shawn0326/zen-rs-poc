use std::cell::RefCell;
use std::rc::Rc;

#[non_exhaustive]
pub struct Material {}

impl Material {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Material {}))
    }
}
