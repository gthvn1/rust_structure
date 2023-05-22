use std::cell::RefCell; // Used to mutate Rc
use std::rc::Rc; // Reference Counted

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}
impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data,
            next: None,
            prev: None,
        }))
    }
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
    front: Pointer<T>,
    back: Pointer<T>,
}

impl<T> Default for DbList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_head = Node::new(data);

        match self.front.take() {
            None => {
                self.back = Some(new_head.clone());
                self.front = Some(new_head);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.as_ref().borrow_mut().next = Some(old_head);
                self.front = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_tail = Node::new(data);

        match self.back.take() {
            None => {
                self.front = Some(new_tail.clone());
                self.back = Some(new_tail);
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.as_ref().borrow_mut().prev = Some(old_tail);
                self.back = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.front.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // consume the prev of the new head
                    new_head.borrow_mut().prev.take();
                    self.front = Some(new_head);
                }
                None => {
                    // We need to consume the tail
                    self.back.take();
                }
            }

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.back.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    // consume the next of the new tail
                    new_tail.borrow_mut().next.take();
                    self.back = Some(new_tail);
                }
                None => {
                    // We need to consume the front
                    self.front.take();
                }
            }

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }
}

impl<T: std::fmt::Display> DbList<T> {
    pub fn stringify(&self) -> String {
        match &self.front {
            None => String::from("List is empty"),
            Some(node) => format!("Front: {}", node.borrow().stringify()),
        }
    }
}
