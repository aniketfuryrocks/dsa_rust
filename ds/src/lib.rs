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

pub mod stacks;
pub mod queues;
pub mod linked_lists;
