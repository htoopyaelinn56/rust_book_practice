use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>, // This can cause a cycle in a doubly linked list
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct NodeWithWeakRef {
    value: i32,
    next: Option<Rc<RefCell<NodeWithWeakRef>>>,
    prev: Option<Weak<RefCell<NodeWithWeakRef>>>, // This can cause a cycle in a doubly linked list
}

impl NodeWithWeakRef {
    fn new(value: i32) -> Rc<RefCell<NodeWithWeakRef>> {
        Rc::new(RefCell::new(NodeWithWeakRef {
            value,
            next: None,
            prev: None,
        }))
    }
}

fn main() {
    memory_leak();
    println!("-----");
    prevent_memory_leak();
}

fn memory_leak() {
    let a = Node::new(1);
    let b = Node::new(2);

    println!("a strong count: {}", Rc::strong_count(&a));
    println!("b strong count: {}", Rc::strong_count(&b));

    // Create strong references between a and b
    a.borrow_mut().next = Some(Rc::clone(&b));
    b.borrow_mut().prev = Some(Rc::clone(&a));

    // At this point, 'a' and 'b' have strong counts of 2 each.
    // Even when 'a' and 'b' go out of scope at the end of main,
    // the nodes they point to will still have strong counts of 1 due to the cycle.
    // The memory for these nodes will not be deallocated.

    println!("a strong count: {}", Rc::strong_count(&a));
    println!("b strong count: {}", Rc::strong_count(&b));
}

fn prevent_memory_leak() {
    let a = NodeWithWeakRef::new(1);
    let b = NodeWithWeakRef::new(2);

    // Create a strong reference from a to b
    a.borrow_mut().next = Some(Rc::clone(&b));
    // Create a weak reference from b to a
    b.borrow_mut().prev = Some(Rc::downgrade(&a));

    if let Some(a_from_b) = b.borrow().prev.as_ref().and_then(|weak| weak.upgrade()) {
        // If upgrade is successful, we get an Option<Rc<RefCell<Node>>>
        // and then we can safely access the data inside the Rc and RefCell
        println!(
            "Successfully accessed 'a' from 'b'. Value: {}",
            a_from_b.borrow().value
        );
    } else {
        println!("Could not access 'a' from 'b'. It might have been dropped.");
    }

    // When 'a' and 'b' go out of scope, their initial Rc references are dropped.
    // The strong count of the node pointed to by 'a' will become 0, and it will be dropped.
    // When the node pointed to by 'a' is dropped, the weak reference from 'b' to 'a' becomes invalid.
    // The strong count of the node pointed to by 'b' will become 0, and it will be dropped.

    // The strong count of 'a' is now 1 (from the 'next' field of b)
    println!("a strong count after linking: {}", Rc::strong_count(&a));
    // The strong count of 'b' is now 1 (from the 'next' field of a)
    println!("b strong count after linking: {}", Rc::strong_count(&b));

    // When a and b go out of scope, their Rc instances are dropped, decreasing the strong count.
    // The weak count of 'a' is now 1 (from the 'prev' field of b)
    println!("a weak count after linking: {}", Rc::weak_count(&a));

    // Now let's drop 'a' to simulate it being deallocated
    drop(a);
    println!("'a' has been dropped.");

    // Try to access 'a' from 'b' again
    println!("Trying to access 'a' from 'b' after dropping 'a'...");
    if let Some(a_from_b) = b.borrow().prev.as_ref().and_then(|weak| weak.upgrade()) {
        // If upgrade is successful, we get an Option<Rc<RefCell<Node>>>
        // and then we can safely access the data inside the Rc and RefCell
        println!(
            "Successfully accessed 'a' from 'b'. Value: {}",
            a_from_b.borrow().value
        );
    } else {
        println!("Could not access 'a' from 'b'. It might have been dropped.");
    };
}
