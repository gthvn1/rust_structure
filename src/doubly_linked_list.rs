// It allows to have mutliple immutable reference that is ok for the
// borrow checker but we can modify them (so in fact we have multiple
// mutable reference).
use std::cell::RefCell;
// Allow multiple ownership using reference counter
// Weak pointers allows to avoid reference cycle (ie if two Rc point
// to each other because in this case they cannot be dropped).
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DbList<T> {
    // todo
    head: Option<Rc<RefCell<DbNode<T>>>>,
    // if there is one element head and tail will point to the same location
    // so we need to use a weak reference for one of them to be able to drop
    // the element.
    tail: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    // Same than head & tail, we need to use a weak reference for one of the
    // two.
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> Default for DbList<T> {
    fn default() -> Self {
        todo!()
    }
}

impl<T> DbList<T> {
    pub fn push_front(&mut self, data: T) {
        todo!();
    }

    pub fn pop_front(&mut self) -> Option<T> {
        todo!();
    }

    pub fn push_back(&mut self, data: T) {
        todo!();
    }
}
