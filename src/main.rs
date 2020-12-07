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

fn main() {
}