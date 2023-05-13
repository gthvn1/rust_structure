pub struct LinkedList(i32);

impl LinkedList {
    pub fn new() -> Self {
        LinkedList(0)
    }

    pub fn get(&self) -> i32 {
        self.0
    }

    pub fn update(&mut self, val: i32) {
        self.0 = val;
    }
}
