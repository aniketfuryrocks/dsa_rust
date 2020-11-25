struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

pub struct SinglyLinkedList<T> {
    size: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn push_front(&mut self, value: T) {
        self.next = Some(Box::new(Node {
            next: self.next.take(),
            value,
        }));
    }
    pub fn push_back(&mut self, value: T) {
        self.next = Some(Box::new(Node {
            next: self.next.take(),
            value,
        }));
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            size: 0,
            head: None,
        }
    }
    pub fn push_front(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            next: self.head.take(),
            value,
        }));
    }
    pub fn push_back(&mut self, value: T) {
        match &mut self.head {
            Some(s) => {
                s.push_back(value)
            }
            _ => {
                self.head = Some(Box::new(Node {
                    next: None,
                    value,
                }))
            }
        }
    }
}

#[test]
fn singly_linked_list_test() {
    let mut my_list = SinglyLinkedList::new();
    my_list.push_front(1);
}