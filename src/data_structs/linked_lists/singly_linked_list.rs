type NextNode<T> = Option<Box<Node<T>>>;

struct Node<T> {
    next: NextNode<T>,
    value: T,
}

struct NodeIter<'a, T> {
    cur: Option<&'a NextNode<T>>
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a Box<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.unwrap() {
            None => { None }
            Some(_) => {
                let to_return = self.cur.take().unwrap().as_ref();
                self.cur = Some(&to_return.unwrap().next);
                return to_return;
            }
        }
    }
}

pub struct SinglyLinkedList<T> {
    size: usize,
    head: NextNode<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList {
            size: 0,
            head: None,
        }
    }
    pub fn push_front(&mut self, value: T) {
        self.size += 1;
        self.head = Some(Box::new(Node {
            next: self.head.take(),
            value,
        }));
    }
    pub fn push_back(&mut self, value: T) {
        self.size += 1;
        let mut k = &mut self.head;
        while let Some(node) = k {
            match node.next {
                None => {
                    node.next = Some(Box::new(Node {
                        next: None,
                        value,
                    }));
                    break;
                }
                _ => {
                    k = &mut node.next;
                }
            }
        }
    }
    pub fn iter(&self) -> NodeIter<'_, T> {
        NodeIter {
            cur: Some(&self.head)
        }
    }
}

#[test]
fn singly_linked_list_test() {
    let mut my_list = SinglyLinkedList::new();
    my_list.push_front(1);
    my_list.push_front(2);
    my_list.push_back(3);
    assert_eq!(my_list.size, 3);
    for x in my_list.iter() {
        println!("{:?}", x.value);
    }
    for x in my_list.iter() {
        println!("{:?}", x.value);
    }
    for x in my_list.iter() {
        println!("{:?}", x.value);
    }
    for x in my_list.iter() {
        println!("{:?}", x.value);
    }
}