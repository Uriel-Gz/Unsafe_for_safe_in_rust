#[cfg(test)]
mod queue_test {
    use structures::queue::Queue;

    #[test]
    fn test_enqueuer_y_desenqueuer() {
        let mut cola: Queue<u8> = Queue::nueva();
        cola.enqueuer(1);
        cola.enqueuer(2);
        cola.enqueuer(3);

        assert_eq!(cola.desenqueuer(), Some(1));
        assert_eq!(cola.desenqueuer(), Some(2));
        assert_eq!(cola.desenqueuer(), Some(3));
        assert_eq!(cola.desenqueuer(), None);
    }

    #[test]
    fn test_es_vacia() {
        let mut cola: Queue<u8> = Queue::nueva();
        assert!(cola.es_vacia());

        cola.enqueuer(1);
        assert!(!cola.es_vacia());

        cola.desenqueuer();
        assert!(cola.es_vacia());
    }

    #[test]
    fn test_enqueuer_en_cola_vacia() {
        let mut cola: Queue<u8> = Queue::nueva();
        cola.enqueuer(10);
        assert_eq!(cola.desenqueuer(), Some(10));
    }

    #[test]
    fn test_desenqueuer_de_cola_vacia() {
        let mut cola: Queue<u8> = Queue::nueva();
        assert_eq!(cola.desenqueuer(), None);
    }

    #[test]
    fn test_enqueuer_y_desenqueuer_multiples_veces() {
        let mut cola: Queue<u8> = Queue::nueva();
        cola.enqueuer(1);
        cola.enqueuer(2);
        cola.enqueuer(3);

        assert_eq!(cola.desenqueuer(), Some(1));
        assert_eq!(cola.desenqueuer(), Some(2));
        assert_eq!(cola.desenqueuer(), Some(3));

        cola.enqueuer(4);
        cola.enqueuer(5);
        cola.enqueuer(6);

        assert_eq!(cola.desenqueuer(), Some(4));
        assert_eq!(cola.desenqueuer(), Some(5));
        assert_eq!(cola.desenqueuer(), Some(6));
    }

    #[test]
    fn test_drop() {
        let mut cola: Queue<u8> = Queue::nueva();
        cola.enqueuer(1);
        cola.enqueuer(2);
        cola.enqueuer(3);

        drop(cola);

        // No se puede acceder a la cola después de drop
    }
}