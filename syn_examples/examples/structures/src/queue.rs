// queue.rs
pub struct Nodo<T> {
    pub valor: T,
    pub siguiente: *mut Nodo<T>,
}

pub struct Queue<T> {
    cabeza: *mut Nodo<T>,
    queue: *mut Nodo<T>,
}

impl<T> Queue<T> {
    pub fn nueva() -> Self {
        Queue {
            cabeza: std::ptr::null_mut(),
            queue: std::ptr::null_mut(),
        }
    }

    pub fn enqueuer(&mut self, valor: T) {
        unsafe {
            let nuevo_nodo = Box::into_raw(Box::new(Nodo {
                valor,
                siguiente: std::ptr::null_mut(),
            }));

            if self.queue.is_null() {
                self.cabeza = nuevo_nodo;
                self.queue = nuevo_nodo;
            } else {
                (*self.queue).siguiente = nuevo_nodo;
                self.queue = nuevo_nodo;
            }
        }
    }

    pub fn desenqueuer(&mut self) -> Option<T> {
        unsafe {
            if self.cabeza.is_null() {
                return None;
            }

            let cabeza_nodo = Box::from_raw(self.cabeza);
            self.cabeza = cabeza_nodo.siguiente;

            if self.cabeza.is_null() {
                self.queue = std::ptr::null_mut();
            }

            Some(cabeza_nodo.valor)
        }
    }

    pub fn es_vacia(&self) -> bool {
        self.cabeza.is_null()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while !self.es_vacia() {
            self.desenqueuer();
        }
    }
}