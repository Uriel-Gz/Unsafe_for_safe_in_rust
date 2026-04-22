// Estructura para un nodo del árbol AVL
pub struct Nodo {
    clave: i32,
    altura: i32,
    izquierda: *mut Nodo,
    derecha: *mut Nodo,
}

impl Nodo {
    // Constructor para un nuevo nodo
    pub fn nuevo(clave: i32) -> *mut Nodo {
        let mut nodo: Box<Nodo> = Box::new(Nodo {
            clave,
            altura: 1,
            izquierda: std::ptr::null_mut(),
            derecha: std::ptr::null_mut(),
        });
        nodo.as_mut() as *mut Nodo
    }

    // Destructor para un nodo
    pub fn destruir(nodo: *mut Nodo) {
        if !nodo.is_null() {
            let nodo: &mut Nodo = unsafe { &mut *nodo };
            if !nodo.izquierda.is_null() {
                Self::destruir(nodo.izquierda);
            }
            if !nodo.derecha.is_null() {
                Self::destruir(nodo.derecha);
            }
            unsafe {
                let _ = Box::from_raw(nodo);
            }
        }
    }

    // Obtener la altura de un nodo
    pub fn obtener_altura(nodo: *mut Nodo) -> i32 {
        if nodo.is_null() {
            0
        } else {
            unsafe {
                let nodo: &mut Nodo = &mut *nodo;
                nodo.altura
            }
        }
    }

    // Actualizar la altura de un nodo
    pub fn actualizar_altura(nodo: *mut Nodo) {
        if !nodo.is_null() {
            unsafe {
                let nodo: &mut Nodo = &mut *nodo;
                nodo.altura = 1 + std::cmp::max(
                    Self::obtener_altura(nodo.izquierda),
                    Self::obtener_altura(nodo.derecha),
                );
            }
        }
    }

    // Rotación a la izquierda
    pub fn rotar_izquierda(nodo: *mut Nodo) -> *mut Nodo {
        if !nodo.is_null() {
            unsafe {
                let nodo: &mut Nodo = &mut *nodo;
                let nuevo_nodo: *mut Nodo = nodo.derecha;
                let temp: *mut Nodo = (*nuevo_nodo).izquierda;
                (*nuevo_nodo).izquierda = nodo;
                (*nodo).derecha = temp;
                Self::actualizar_altura(nodo);
                Self::actualizar_altura(nuevo_nodo);
                nuevo_nodo
            }
        } else {
            nodo
        }
    }

    // Rotación a la derecha
    pub fn rotar_derecha(nodo: *mut Nodo) -> *mut Nodo {
        if !nodo.is_null() {
            unsafe {
                let nodo: &mut Nodo = &mut *nodo;
                let nuevo_nodo: *mut Nodo = nodo.izquierda;
                let temp: *mut Nodo = (*nuevo_nodo).derecha;
                (*nuevo_nodo).derecha = nodo;
                (*nodo).izquierda = temp;
                Self::actualizar_altura(nodo);
                Self::actualizar_altura(nuevo_nodo);
                nuevo_nodo
            }
        } else {
            nodo
        }
    }

    // Balancear el árbol
    pub fn balancear(nodo: *mut Nodo) -> *mut Nodo {
        if !nodo.is_null() {
            let altura_izquierda: i32 = Self::obtener_altura(nodo);
            let altura_derecha: i32 = Self::obtener_altura(nodo);
            if altura_izquierda > altura_derecha + 1 {
                if Self::obtener_altura(nodo) >= Self::obtener_altura(nodo) {
                    Self::rotar_derecha(nodo)
                } else {
                    unsafe {
                        let nodo: &mut Nodo = &mut *nodo;
                        (*nodo).izquierda = Self::rotar_izquierda((*nodo).izquierda);
                    }
                    Self::rotar_derecha(nodo)
                }
            } else if altura_derecha > altura_izquierda + 1 {
                if Self::obtener_altura(nodo) >= Self::obtener_altura(nodo) {
                    Self::rotar_izquierda(nodo)
                } else {
                    unsafe {
                        let nodo: &mut Nodo = &mut *nodo;
                        (*nodo).derecha = Self::rotar_derecha((*nodo).derecha);
                    }
                    Self::rotar_izquierda(nodo)
                }
            } else {
                nodo
            }
        } else {
            nodo
        }
    }

    // Insertar un nuevo nodo en el árbol
    pub fn insertar(nodo: *mut Nodo, clave: i32) -> *mut Nodo {
        if nodo.is_null () {
            Nodo::nuevo(clave)
        } else {
            unsafe {
                let nodo: &mut Nodo = &mut *nodo;
                if clave < (*nodo).clave {
                    if !nodo.izquierda.is_null() {
                        nodo.izquierda = Self::insertar(nodo.izquierda, clave);
                    }
                } else {
                    if !nodo.derecha.is_null() {
                        nodo.derecha = Self::insertar(nodo.derecha, clave);
                    }
                }
                Self::actualizar_altura(nodo);
                Self::balancear(nodo)
            }
        }
    }

    // Función para imprimir el árbol en orden
    pub fn imprimir_en_orden(nodo: *mut Nodo) {
        if !nodo.is_null() {
            unsafe {
                if !(*nodo).izquierda.is_null() {
                    Self::imprimir_en_orden((*nodo).izquierda);
                }
                if !nodo.is_null() {
                    println!("{}", (*nodo).clave);
                }
                if !(*nodo).derecha.is_null() {
                    Self::imprimir_en_orden((*nodo).derecha);
                }
            }
        }
    }
}

// Estructura para el árbol AVL
pub struct ArbolAVL {
    pub raiz: *mut Nodo,
}

impl ArbolAVL {
    // Constructor para un nuevo árbol AVL
    pub fn nuevo() -> ArbolAVL {
        ArbolAVL { raiz: std::ptr::null_mut() }
    }

    // Insertar un nuevo nodo en el árbol
    pub fn insertar(&mut self, clave: i32) {
        self.raiz = Nodo::insertar(self.raiz, clave);
    }

    // Imprimir el árbol en orden
    pub fn imprimir(&self) {
        Nodo::imprimir_en_orden(self.raiz);
    }

    pub fn destruir(&mut self) {
        Nodo::destruir(self.raiz);
        self.raiz = std::ptr::null_mut(); // Actualiza la raíz del árbol AVL
    }
}
