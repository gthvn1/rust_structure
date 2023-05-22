use rust_structures::binary_tree::*;
use rust_structures::doubly_linked_list::*;
use rust_structures::linked_list::*;

fn try_binary_tree() {
    println!("\n================================================");
    println!("== Binary tree\n ");
    let mut bt: BTree<i32> = BTree::default();

    bt.add(4);
    bt.add(32);
    bt.add(12);
    bt.add(3);
    bt.add(67);
    bt.add(9);
    bt.add(1);
    bt.add(45);

    bt.print_lfirst(0);

    println!("Applying Right rotation");
    bt.rrot();
    bt.print_lfirst(0);
}

fn try_doubly_linked_list() {
    println!("\n================================================");
    println!("== Doubly Linked List\n");
    let mut dbl = DbList::default();

    let data = 30;
    println!("push {} to front", data);
    dbl.push_front(data);
    println!("=> {}", dbl.stringify());

    let data = 20;
    println!("push {} to back", data);
    dbl.push_back(data);
    println!("=> {}", dbl.stringify());

    let data = 40;
    println!("push {} to front", data);
    dbl.push_front(data);
    println!("=> {}", dbl.stringify());

    let data = dbl.pop_back();
    println!("pop {:?} from the back", data);
    println!("=> {}", dbl.stringify());

    let data = dbl.pop_front();
    println!("pop {:?} from the front", data);
    println!("=> {}", dbl.stringify());

    let data = dbl.pop_front();
    println!("pop {:?} from the front", data);
    println!("=> {}", dbl.stringify());
}

fn try_linked_list() {
    println!("\n================================================");
    println!("== Linked List\n");

    let mut ll: LinkedList<i32> = LinkedList::default();
    println!("create linked list : {:?}", ll);

    ll.push_front(10);
    println!("push 10 in front   : {:?}", ll);

    ll.push_front(5);
    println!("push 5 in front    : {:?}", ll);

    ll.push_back(20);
    println!("push 20 in tail    : {:?}", ll);

    ll.push_sorted(12);
    println!("sorted push 12     : {:?}", ll);

    let data = ll.pop_front();
    println!("{:?} popped in front: {:?}", data, ll);
}

fn main() {
    try_binary_tree();
    try_linked_list();
    try_doubly_linked_list();
}
