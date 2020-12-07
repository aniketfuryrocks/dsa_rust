use std::collections::LinkedList;
use std::f32::consts::E;

type NextNode<T> = Option<Box<Node<T>>>;

struct Node<T> {
    next: NextNode<T>,
    value: T,
}

pub struct NodeIter<'a, T> {
    cur: Option<&'a NextNode<T>>
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.unwrap() {
            None => { None }
            Some(_) => {
                let to_return = self.cur.take().unwrap().as_ref();
                self.cur = Some(&to_return.unwrap().next);
                return Some(&to_return.unwrap().value);
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
        let last_node = Some(Box::new(Node {
            next: None,
            value,
        }));
        while let Some(node) = k {
            match node.next {
                None => {
                    node.next = last_node;
                    return;
                }
                _ => {
                    k = &mut node.next;
                }
            }
        }
        self.head = last_node;
    }
    pub fn iter(&self) -> NodeIter<'_, T> {
        NodeIter {
            cur: Some(&self.head)
        }
    }
    pub fn get(&self, index: usize) -> Result<&T, &'static str> {
        let mut k = &self.head;
        if k.is_none() {
            return Err("Index Out of Bounds");
        }
        let mut i: usize = 0;
        loop {
            let node = k.as_ref().unwrap();
            if i == index {
                break Ok(&node.value);
            }
            i += 1;
            if node.next.is_none() {
                break Err("Index Out of Bounds");
            } else {
                k = &node.next;
            }
        }
    }
    pub fn set(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index Out of Bounds");
        }
        if index == 0 {
            return Ok(self.push_front(value));
        }
        self.size += 1;
        let mut node = self.head.as_mut().unwrap();
        //TODO: optimize for tail
        let mut i: usize = 0;
        loop {
            i += 1;
            if i == index {
                node.next = Some(Box::new(Node {
                    next: node.next.take(),
                    value,
                }));
                break Ok(());
            }
            node = node.next.as_mut().unwrap();
        }
    }

    pub fn delete(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index Out of Bounds");
        }
        self.size -= 1;
        if index == 0 {
            let node = self.head.take().unwrap();
            self.head = node.next;
        }
        //TODO: optimize for tail
        let mut node = self.head.as_mut().unwrap();
        let mut i: usize = 0;
        loop {
            i += 1;
            if i == index {
                let next_node = node.next.take().unwrap();
                node.next = next_node.next;
                break Ok(());
            }
            node = node.next.as_mut().unwrap();
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
        println!("{:?}", x);
    }
    my_list.set(0, 0);
    assert_eq!(*my_list.get(0).unwrap(), 0);
    assert_eq!(*my_list.get(1).unwrap(), 2);
    assert_eq!(*my_list.get(2).unwrap(), 1);
    assert_eq!(*my_list.get(3).unwrap(), 3);
    my_list.get(12).expect_err("Index Out of Bounds");
}