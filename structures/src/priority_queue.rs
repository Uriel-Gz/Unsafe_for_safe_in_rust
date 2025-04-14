pub struct PriorityQueue<T> {
   pub elements: Vec<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
        unsafe {
            self.heapify_up(0);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            return None;
        }
        let last_index = self.elements.len() - 1;
        self.elements.swap(0, last_index);
        let elem: Option<T>;
        if self.elements[last_index] > self.elements[0] {
            elem = self.elements.pop();
            unsafe {
                self.heapify_down(0);
            }
        } else {
            elem = self.elements.pop();
            unsafe {
                self.heapify_up(0);
            }
        }

        elem
    }

    pub unsafe fn heapify_up(&mut self, index: usize) {
        let mut current_index = index;
        let len = self.elements.len();
        loop {
            let left_child_index = 2 * current_index + 1;
            let right_child_index = 2 * current_index + 2;
            let mut largest_index = current_index;

            if left_child_index < len && self.elements[left_child_index] < self.elements[largest_index] {
                largest_index = left_child_index;
            }
            if right_child_index < len && self.elements[right_child_index] < self.elements[largest_index] {
                largest_index = right_child_index;
            }
            if largest_index == current_index {
                break;
            }
            self.elements.swap(current_index, largest_index);
            current_index = largest_index;
        }
    }

    pub unsafe fn heapify_down(&mut self, index: usize) {
        let mut current_index = index;
        let len = self.elements.len();
        loop {
            let left_child_index = 2 * current_index + 1;
            let right_child_index = 2 * current_index + 2;
            let mut largest_index = current_index;

            if left_child_index < len && self.elements[left_child_index] > self.elements[largest_index] {
                largest_index = left_child_index;
            }
            if right_child_index < len && self.elements[right_child_index] > self.elements[largest_index] {
                largest_index = right_child_index;
            }
            if largest_index == current_index {
                break;
            }
            self.elements.swap(current_index, largest_index);
            current_index = largest_index;
        }
    }
}