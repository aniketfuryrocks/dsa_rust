use core::ptr;

queue_struct!(Queue);
impl_que_new!(Queue);

impl<T> Queue<T> {
    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            Err("Overflow")
        } else {
            self.arr[self.rear] = item;
            self.rear += 1;
            self.size = self.rear;
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
        for i in 0..5 {
            q.enqueue(i).unwrap();
        }
        for i in 0..5 {
            assert_eq!(q.dequeue().unwrap(), i);
        }
        match q.dequeue() {
            Some(_) => panic!("Should have returned None"),
            _ => {}
        }
        // since this is not a circular que an exception must be thrown
        match q.enqueue(0) {
            Ok(_) => panic!("Should have thrown a overflow error"),
            _ => ()
        }
    };
}