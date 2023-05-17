#[derive(Debug)]
pub struct BNode<T> {
    data: T,
    height: i8,
    left: BTree<T>,
    right: BTree<T>,
}

impl<T> BNode<T> {
    pub fn new(data: T) -> Self {
        BNode {
            data,
            height: 0,
            left: BTree::new(),
            right: BTree::new(),
        }
    }

    fn right_rotation(mut self) -> Box<Self> {
        let new_node = self.left.0.take();

        match new_node {
            None => return Box::new(self),
            Some(mut node) => {
                let old_left = node.right.0.take();
                self.left = BTree(old_left);
                self.left.update_height();

                node.right = BTree(Some(Box::new(self)));
                node.right.update_height();
                node
            }
        }
    }
}

#[derive(Debug)]
pub struct BTree<T>(Option<Box<BNode<T>>>);

impl<T> Default for BTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BTree<T> {
    pub fn new() -> Self {
        BTree(None)
    }

    pub fn height(&self) -> i8 {
        match &self.0 {
            None => 0,
            Some(node) => node.height,
        }
    }

    pub fn update_height(&mut self) {
        if let Some(node) = &mut self.0 {
            node.height = 1 + std::cmp::max(node.left.height(), node.right.height());
        }
    }

    pub fn rrot(&mut self) {
        if let Some(node) = self.0.take() {
            self.0 = Some(node.right_rotation());
        }
    }
}

impl<T: PartialOrd> BTree<T> {
    pub fn add(&mut self, data: T) {
        match self.0 {
            Some(ref mut node) => {
                if node.data > data {
                    node.left.add(data);
                } else {
                    node.right.add(data);
                }
            }
            None => {
                self.0 = Some(Box::new(BNode::new(data)));
            }
        }

        // We need to compute the new height of the node
        // The deepest node will start by 1.
        self.update_height();
    }
}

impl<T: std::fmt::Debug> BTree<T> {
    pub fn print_lfirst(&self, dp: u32) {
        if let Some(ref node) = self.0 {
            node.left.print_lfirst(dp + 1);
            let mut indent = String::new();
            for _ in 0..dp {
                indent.push('-');
            }
            println!("{}: {}{:?}", node.height, indent, node.data);
            node.right.print_lfirst(dp + 1);
        }
    }
}
