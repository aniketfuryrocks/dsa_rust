#[macro_export]
macro_rules! impl_que_new {
    ($name:ident) => {
        impl<T> $name<T> {
            pub unsafe fn new(size: usize) -> $name<T> {
                let mut q = $name {
                    arr: Vec::with_capacity(size),
                    front: 0,
                    rear: 0,
                    size: 0
                };
                q.arr.set_len(size);
                q
            }
            pub fn size(&self) -> usize {
                self.size
            }
            pub fn is_full(&self) -> bool {
                self.size == self.arr.len()
            }
            pub fn is_empty(&self) -> bool {
                self.size == 0
            }
        }
    };
}

#[macro_export]
macro_rules! queue_struct {
    ($name:ident) => {
        pub struct $name<T> {
            arr: Vec<T>,
            front: usize,
            rear: usize,
            size: usize
        }
    };
}

pub mod circular_queue;
pub mod queue;
