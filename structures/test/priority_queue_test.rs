#[cfg(test)]
mod tests {
    use structures::priority_queue::PriorityQueue;

    #[test]
    fn test_crear_cola_de_prioridad_vacia() {
        let pq: PriorityQueue<i32> = PriorityQueue::new();
        assert!(pq.elements.is_empty());
    }

    #[test]
    fn test_push_elemento_en_cola_de_prioridad() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        pq.push(5);
        assert_eq!(pq.elements.len(), 1);
        assert_eq!(pq.elements[0], 5);
    }

    #[test]
    fn test_push_varios_elementos_en_cola_de_prioridad() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        pq.push(5);
        pq.push(10);
        pq.push(3);
        assert_eq!(pq.elements.len(), 3);
        assert_eq!(pq.elements[0], 10);
        assert_eq!(pq.elements[1], 5);
        assert_eq!(pq.elements[2], 3);
    }

    #[test]
    fn test_pop_elemento_de_cola_de_prioridad() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        pq.push(5);
        assert_eq!(pq.pop(), Some(5));
        assert!(pq.elements.is_empty());
    }

    #[test]
    fn test_pop_varios_elementos_de_cola_de_prioridad() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        pq.push(5);
        pq.push(10);
        pq.push(3);
        assert_eq!(pq.pop(), Some(10));
        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.pop(), Some(3));
        assert!(pq.elements.is_empty());
    }

    #[test]
    fn test_pop_cola_de_prioridad_vacia() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        assert!(pq.pop().is_none());
    }

    #[test]
    fn test_heapify_up_en_cola_de_prioridad() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        pq.elements.push(5);
        pq.elements.push(10);
        pq.elements.push(3);
        unsafe {
            pq.heapify_up(2);
        }
        assert_eq!(pq.elements[0], 10);
        assert_eq!(pq.elements[1], 5);
        assert_eq!(pq.elements[2], 3);
    }

    #[test]
    fn test_heapify_down_en_cola_de_prioridad() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        pq.elements.push(10);
        pq.elements.push(5);
        pq.elements.push(3);
        unsafe {
            pq.heapify_down(0);
        }
        assert_eq!(pq.elements[0], 10);
        assert_eq!(pq.elements[1], 5);
        assert_eq!(pq.elements[2], 3);
    }

    #[test]
    fn test_cola_de_prioridad_con_elementos_repetidos() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        pq.push(5);
        pq.push(5);
        pq.push(10);
        pq.push(3);
        assert_eq!(pq.pop(), Some(10));
        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.pop(), Some(3));
        assert!(pq.elements.is_empty());
    }

    #[test]
    fn test_cola_de_prioridad_con_elementos_negativos() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        pq.push(-5);
        pq.push(10);
        pq.push(-3);
        assert_eq!(pq.pop(), Some(10));
        assert_eq!(pq.pop(), Some(-3));
        assert_eq!(pq.pop(), Some(-5));
        assert!(pq.elements.is_empty());
    }
}