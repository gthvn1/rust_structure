#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList(None)
    }
}

impl<T: PartialOrd> LinkedList<T> {
    pub fn push_front(&mut self, data: T) {
        let prev_head = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(prev_head))));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.0.take() {
            None => None,
            Some((data, child)) => {
                self.0 = child.0;
                Some(data)
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match &mut self.0 {
            None => self.push_front(data),
            Some((_, child)) => {
                child.push_back(data);
            }
        }
    }

    pub fn push_sorted(&mut self, data: T) {
        match &mut self.0 {
            None => self.push_front(data),
            Some((d, child)) => {
                if data > *d {
                    child.push_sorted(data);
                } else {
                    self.push_front(data);
                }
            }
        }
    }
}
