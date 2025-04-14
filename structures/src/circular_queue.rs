pub struct CircularQueue<T> {
    pub data: Vec<T>,
    pub head: usize,
    pub tail: usize,
    pub size: usize,
}

impl<T: Clone> CircularQueue<T> {
    pub fn new(capacity: usize) -> Self {
        CircularQueue {
            data: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        if self.data.len() == 0 || self.data.len() < self.data.capacity(){
            self.data.push(item);
            self.tail = (self.tail + 1) % self.data.capacity();
        } else {
            self.data[self.tail] = item;
            self.tail = (self.tail + 1) % self.data.capacity();
        }
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let item = self.data[self.head].clone();
            self.head = (self.head + 1) % self.data.capacity();
            self.size -= 1;
            Some(item)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.data.capacity()
    }
}