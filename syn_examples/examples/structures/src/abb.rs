// Definimos el nodo del árbol
pub struct Nodo {
    pub valor: i32,
    pub izquierda: *mut Nodo,
    pub derecha: *mut Nodo,
}

impl Nodo {
    // Creamos un nuevo nodo
    fn nuevo(valor: i32) -> *mut Nodo {
        let mut nodo = Box::new(Nodo {
            valor,
            izquierda: std::ptr::null_mut(),
            derecha: std::ptr::null_mut(),
        });
        nodo.as_mut() as *mut Nodo
    }
}

// Definimos el árbol binario de búsqueda
pub struct BinarySearchTree {
    pub raiz: *mut Nodo,
}

impl BinarySearchTree {
    // Creamos un nuevo árbol binario de búsqueda
    pub fn nuevo() -> BinarySearchTree {
        BinarySearchTree { raiz: std::ptr::null_mut() }
    }

    // Insertamos un nuevo valor en el árbol
    pub unsafe fn insertar(&mut self, valor: i32) {
        let nuevo_nodo = Nodo::nuevo(valor);
        if self.raiz.is_null() {
            self.raiz = nuevo_nodo;
        } else {
            self.insertar_recursivo(self.raiz, nuevo_nodo);
        }
    }

    // Insertamos un nuevo valor en el árbol de manera recursiva
    unsafe fn insertar_recursivo(&self, actual: *mut Nodo, nuevo_nodo: *mut Nodo) {
        if (*nuevo_nodo).valor < (*actual).valor {
            if (*actual).izquierda.is_null() {
                (*actual).izquierda = nuevo_nodo;
            } else {
                self.insertar_recursivo((*actual).izquierda, nuevo_nodo);
            }
        } else {
            if (*actual).derecha.is_null() {
                (*actual).derecha = nuevo_nodo;
            } else {
                self.insertar_recursivo((*actual).derecha, nuevo_nodo);
            }
        }
    }

    // Buscamos un valor en el árbol
    pub unsafe fn buscar(&self, valor: i32) -> bool {
        self.buscar_recursivo(self.raiz, valor)
    }

    // Buscamos un valor en el árbol de manera recursiva
    unsafe fn buscar_recursivo(&self, actual: *mut Nodo, valor: i32) -> bool {
        if actual.is_null() {
            false
        } else if (*actual).valor == valor {
            true
        } else if valor < (*actual).valor {
            self.buscar_recursivo((*actual).izquierda, valor)
        } else {
            self.buscar_recursivo((*actual).derecha, valor)
        }
    }

    fn drop_recursivo(&self, nodo: *mut Nodo) {
        if !nodo.is_null() {
            self.drop_recursivo(unsafe { (*nodo).izquierda });
            self.drop_recursivo(unsafe { (*nodo).derecha });
            let _ = unsafe { std::ptr::read(nodo) };
            // unsafe { std::ptr::drop(nodo) };
            unsafe {
                std::alloc::dealloc(nodo as *mut u8, std::alloc::Layout::new::<Nodo>());
            }
        }
    }
}

impl Drop for BinarySearchTree {
    fn drop(&mut self) {
        self.drop_recursivo(self.raiz);
    }
}