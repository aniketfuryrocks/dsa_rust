use core::ptr;

queue_struct!(Queue);
impl_que_new!(Queue);

impl<T> Queue<T> {
    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if self.rear == self.arr.len() {
            Err("Overflow")
        } else {
            self.arr[self.rear] = item;
            self.rear += 1;
            Ok(())
        }
    }

    pub unsafe fn dequeue(&mut self) -> Option<T> {
        if self.front == self.rear {
            None
        } else {
            let dequeued = ptr::read(self.arr.as_ptr().add(self.front));
            self.front += 1;
            Some(dequeued)
        }
    }
}

#[test]
fn queue_test() {
    unsafe {
        let mut q = Queue::new(5);
        for i in 1..5 {
            q.enqueue(i).unwrap();
        }
        for i in 1..5 {
            assert_eq!(q.dequeue().unwrap(), i);
        }
        match q.dequeue() {
            Some(_) => panic!("Should have returned None"),
            _ => {}
        }
        q.enqueue(5).unwrap();
        // since this is not a circular que an exception must be thrown
        match q.enqueue(6) {
            Ok(_) => panic!("Should have thrown a overflow error"),
            _ => ()
        }
    };
}