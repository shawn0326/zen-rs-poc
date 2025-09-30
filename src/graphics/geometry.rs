use std::cell::RefCell;
use std::rc::Rc;

#[non_exhaustive]
pub struct Geometry {}

impl Geometry {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Geometry {}))
    }
}
