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
}

#[derive(Debug)]
pub struct BTree<T>(Option<Box<BNode<T>>>);

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
