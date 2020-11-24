use core::ptr;

queue_struct!(CircularQueue);
impl_que_new!(CircularQueue);

impl<T> CircularQueue<T> {
    pub fn enqueue(&mut self, item: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Overflow");
        }
        self.arr[self.rear] = item;
        self.rear = circular_increment!(self.rear,self.arr.len());
        self.size += 1;
        Ok(())
    }

    pub unsafe fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let dequeued = ptr::read(self.arr.as_ptr().add(self.front));
        self.front = circular_increment!(self.front,self.arr.len());
        self.size -= 1;
        Some(dequeued)
    }
}

#[test]
fn queue_test() {
    unsafe {
        let mut q = CircularQueue::new(5);
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
        for i in 0..5 {
            q.enqueue(i).unwrap();
        }
        for i in 0..5 {
            assert_eq!(q.dequeue().unwrap(), i);
        }
    };
}