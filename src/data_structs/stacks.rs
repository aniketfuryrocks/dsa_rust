use core::ptr;

pub struct Stack<T> {
    arr: Vec<T>,
    len: usize,
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Stack<T> {
        let mut stack = Stack {
            arr: Vec::with_capacity(size),
            len: 0,
        };
        unsafe {
            stack.arr.set_len(size);
        }
        stack
    }

    pub fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.len == self.arr.len() {
            // though vec can be increased in runtime, yet this implementation is focused to a naive structure
            Err("Stack Overflow")
        } else {
            self.arr[self.len] = item;
            self.len += 1;
            Ok(())
        }
    }
    pub fn pop(&mut self) -> Result<T, &'static str> {
        if self.len == 0 {
            Err("Stack Underflow")
        } else {
            self.len -= 1;
            // though reference can be returned but a naive stack structure returns the value itself
            unsafe {
                Ok(ptr::read(self.arr.as_ptr().add(self.len)))
            }
        }
    }
}

#[test]
fn stack_test() {
    let mut stack = Stack::new(20);
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    stack.push(3).unwrap();
    assert_eq!(stack.pop().unwrap(), 3, "pop");
    assert_eq!(stack.pop().unwrap(), 2, "pop");
    assert_eq!(stack.pop().unwrap(), 1, "pop");
}