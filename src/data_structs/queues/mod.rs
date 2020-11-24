#[macro_export]
macro_rules! impl_que_new {
    ($name:ident) => {
        impl<T> $name<T> {
            pub unsafe fn new(size: usize) -> $name<T> {
                let mut q = $name {
                    arr: Vec::with_capacity(size),
                    front: 0,
                    rear: 0,
                };
                q.arr.set_len(size);
                q
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
        }
    };
}

mod circular_queue;
mod queue;