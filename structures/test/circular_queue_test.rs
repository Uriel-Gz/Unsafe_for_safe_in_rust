#[cfg(test)]
mod circular_queue_test {
    use structures::circular_queue::CircularQueue;

    #[test]
    fn test_crear_cola_circular_vacia() {
        let cola: CircularQueue<i32> = CircularQueue::new(5);
        assert!(cola.is_empty());
        assert_eq!(cola.size, 0);
    }

    #[test]
    fn test_crear_cola_circular_con_elementos() {
        let mut cola: CircularQueue<i32> = CircularQueue::new(5);
        cola.enqueue(1);
        cola.enqueue(2);
        cola.enqueue(3);
        assert!(!cola.is_empty());
        assert_eq!(cola.size, 3);
    }

    #[test]
    fn test_crear_cola_circular_llena() {
        let mut cola: CircularQueue<i32> = CircularQueue::new(1);
        cola.enqueue(1);
        assert!(cola.is_full());
        assert_eq!(cola.size, 1);
    }

    #[test]
    fn test_crear_cola_circular_con_elementos_repetidos() {
        let mut cola: CircularQueue<i32> = CircularQueue::new(5);
        cola.enqueue(1);
        cola.enqueue(2);
        cola.enqueue(3);
        cola.enqueue(1);
        cola.enqueue(2);
        assert!(!cola.is_empty());
        assert_eq!(cola.size, 5);
    }

    #[test]
    fn test_crear_cola_circular_con_muchos_elementos() {
        let mut cola: CircularQueue<i32> = CircularQueue::new(10);
        for i in 1..11 {
            cola.enqueue(i);
        }
        assert!(!cola.is_empty());
        assert_eq!(cola.size, 10);
    }
}