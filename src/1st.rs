use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedList<T: Debug> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(ref node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}
