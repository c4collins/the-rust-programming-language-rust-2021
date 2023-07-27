use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
pub enum RefCountList {
    Cons(i32, Rc<RefCountList>),
    Nil,
}

#[derive(Debug)]
pub enum RefCellRefCountList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellRefCountList>),
    Nil,
}

// Circular References
type CycleRefCell = RefCell<Rc<CycleList>>;

#[derive(Debug)]
pub enum CycleList {
    Cons(i32, CycleRefCell),
    Nil,
}

impl CycleList {
    pub fn tail(&self) -> Option<&CycleRefCell> {
        match self {
            CycleList::Cons(_, item) => Some(item),
            CycleList::Nil => None,
        }
    }
}
