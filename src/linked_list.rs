#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let node: Node = Node {
        data: 0,
        next: Some(Box::from(Node {
            data: 1,
            next: Some(Box::from(Node {
                data: 2,
                next: None,
            })),
        })),
    };

    let mut current_node_ref = &node;
    loop {
        println!("Data: {}", current_node_ref.data);
        if let Some(ref next_boxed_node) = current_node_ref.next {
            current_node_ref = next_boxed_node;
        } else {
            break;
        }
    }
}
