use std::ptr;

pub struct Stack<T> {
    capacidad: usize,
    tamano: usize,
    elementos: *mut T, // Puntero a los datos
}

impl<T> Stack<T> {
    pub fn nueva(capacidad: usize) -> Self {
        let elementos = unsafe {
            let ptr = std::alloc::alloc_zeroed(std::alloc::Layout::array::<T>(capacidad).expect("REASON"));
            if ptr.is_null() {
                panic!("Fallo al asignar memoria");
            }
            ptr as *mut T
        };

        Stack {
            capacidad,
            tamano: 0,
            elementos,
        }
    }

    pub fn empujar(&mut self, elemento: T) {
        if self.tamano == self.capacidad {
            panic!("La Stack está llena");
        }
        unsafe {
            let direccion = self.elementos.add(self.tamano);
            ptr::write(direccion, elemento);
        }
        self.tamano += 1;
    }

    pub fn capacidad(&self) -> usize {
        self.capacidad
    }

    pub fn sacar(&mut self) -> Option<T> {
        if self.tamano == 0 {
            return None;
        }
        let elemento = unsafe {
            let direccion = self.elementos.add(self.tamano - 1);
            Some(ptr::read(direccion))
        };
        self.tamano -= 1;
        elemento
    }

    pub fn tope(&self) -> Option<&T> {
        if self.tamano == 0 {
            return None;
        }
        unsafe {
            let direccion = self.elementos.add(self.tamano - 1);
            Some(&*direccion)
        }
    }

    pub fn vaciar(&mut self) {
        while self.tamano > 0 {
            self.sacar();
        }
    }

    pub fn tamano(&self) -> usize {
        self.tamano
    }

    pub fn es_vacia(&self) -> bool {
        self.tamano == 0
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.tamano {
                let direccion = self.elementos.add(i);
                ptr::drop_in_place(direccion);
            }
            std::alloc::dealloc(
                self.elementos as *mut u8,
                std::alloc::Layout::array::<T>(self.capacidad).expect("REASON"),
            );
        }
    }
}