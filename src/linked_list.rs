#[derive(Debug)]
pub struct LinkedList<T: PartialOrd + std::fmt::Debug>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + std::fmt::Debug> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList(None)
    }
}

impl<T: PartialOrd + std::fmt::Debug> LinkedList<T> {
    pub fn push_front(&mut self, data: T) {
        let prev_head = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(prev_head))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            None => self.push_front(data),
            Some((_, ref mut child)) => {
                child.push_back(data);
            }
        }
    }

    pub fn push_sorted(&mut self, data: T) {
        match self.0 {
            None => self.push_front(data),
            Some((ref d, ref mut child)) => {
                if data > *d {
                    child.push_sorted(data);
                } else {
                    self.push_front(data);
                }
            }
        }
    }
}
