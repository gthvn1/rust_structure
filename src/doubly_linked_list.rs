use std::cell::RefCell; // Used to mutate Rc
use std::rc::Rc; // Reference Counted

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

impl<T: std::fmt::Display> Node<T> {
    fn stringify(&self) -> String {
        match &self.next {
            None => format!("{}", self.data),
            Some(next) => {
                format!("{} -> {}", self.data, next.borrow().stringify())
            }
        }
    }
}

pub struct DbList<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

impl<T> Default for DbList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_head(&mut self, data: T) {
        match self.head.take() {
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: None,
                    prev: None,
                }));
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
            Some(old_head) => {
                let new_node = Rc::new(RefCell::new(Node {
                    data,
                    next: Some(Rc::clone(&old_head)),
                    prev: None,
                }));

                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }
    }
}

impl<T: std::fmt::Display> DbList<T> {
    pub fn stringify(&self) -> String {
        match &self.head {
            None => String::from("List is empty"),
            Some(node) => format!("HEAD: {}", node.borrow().stringify()),
        }
    }
}
