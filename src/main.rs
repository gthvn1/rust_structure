use rust_structures::linked_list::*;

fn main() {
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
