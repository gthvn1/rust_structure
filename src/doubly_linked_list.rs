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
        Self {
            head: None,
            tail: None,
        }
    }
}

impl<T> DbList<T> {
    pub fn push_front(&mut self, data: T) {
        match self.head.take() {
            None => {
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            Some(old_head) => {
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(old_head.clone()),
                    prev: None,
                }));
                // Now we need to modify the former head.
                // So add new_node as a previous weak reference.
                let mut m = old_head.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_node));
                // and we update the head to point to the new node
                self.head = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.tail.take() {
            None => {
                // It is the same as push front
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
            Some(old_tail) => {
                let new_node = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: Some(old_tail.clone()),
                }));

                let st = Weak::upgrade(&old_tail).unwrap();
                let mut m = st.borrow_mut();
                self.tail = Some(Rc::downgrade(&new_node));
                m.next = Some(new_node);
            }
        }
    }
}
