#[macro_export]
macro_rules! circular_increment {
    ($item:expr,$upper:expr) => {
        if $upper -1 == $item {
            0
        } else {
            $item + 1
        }
    };
}

mod algos;
mod data_structs;

use crate::data_structs::linked_lists::singly_linked_list::SinglyLinkedList;
use std::collections::LinkedList;

fn main() {
    let mut sl = SinglyLinkedList::new();
    sl.push_back(12);
    sl.push_back(21);
    sl.push_front(1);
    for x in sl.iter() {
        println!("{:?}", x);
    }
}