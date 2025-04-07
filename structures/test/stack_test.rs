#[cfg(test)]
mod stack_test {
    use structures::stack::Stack;

    #[test]
    fn test_nueva_pila() {
        let pila: Stack<i32> = Stack::nueva(10);
        assert_eq!(pila.capacidad(), 10);
        assert_eq!(pila.tamano(), 0);
    }

    #[test]
    fn test_empujar_elemento() {
        let mut pila: Stack<i32> = Stack::nueva(10);
        pila.empujar(5);
        assert_eq!(pila.tamano(), 1);
    }

    #[test]
    #[should_panic]
    fn test_empujar_elemento_en_pila_llena() {
        let mut pila: Stack<i32> = Stack::nueva(1);
        pila.empujar(5);
        pila.empujar(10);
    }

    #[test]
    fn test_sacar_elemento() {
        let mut pila: Stack<i32> = Stack::nueva(10);
        pila.empujar(5);
        let elemento = pila.sacar().unwrap();
        assert_eq!(elemento, 5);
        assert_eq!(pila.tamano(), 0);
    }

    #[test]
    fn test_sacar_elemento_de_pila_vacia() {
        let mut pila: Stack<i32> = Stack::nueva(10);
        let elemento = pila.sacar();
        assert!(elemento.is_none());
    }

    #[test]
    fn test_tope() {
        let mut pila: Stack<i32> = Stack::nueva(10);
        pila.empujar(5);
        let tope = pila.tope().unwrap();
        assert_eq!(*tope, 5);
    }

    #[test]
    fn test_tope_de_pila_vacia() {
        let pila: Stack<i32> = Stack::nueva(10);
        let tope = pila.tope();
        assert!(tope.is_none());
    }

    #[test]
    fn test_vaciar() {
        let mut pila: Stack<i32> = Stack::nueva(10);
        pila.empujar(5);
        pila.empujar(10);
        pila.vaciar();
        assert_eq!(pila.tamano(), 0);
    }

    #[test]
    fn test_tamano() {
        let mut pila: Stack<i32> = Stack::nueva(10);
        pila.empujar(5);
        pila.empujar(10);
        assert_eq!(pila.tamano(), 2);
    }

    #[test]
    fn test_es_vacia() {
        let pila: Stack<i32> = Stack::nueva(10);
        assert!(pila.es_vacia());
    }

    #[test]
    fn test_no_es_vacia() {
        let mut pila: Stack<i32> = Stack::nueva(10);
        pila.empujar(5);
        assert!(!pila.es_vacia());
    }
}