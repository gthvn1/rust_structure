use rust_structures::linked_list::*;

#[test]
fn create_linked_list() {
    let ll = LinkedList::new();

    assert_eq!(ll.get(), 0);
}

#[test]
fn update_linked_list() {
    let mut ll = LinkedList::new();

    ll.update(42);
    assert_eq!(ll.get(), 42);
}
