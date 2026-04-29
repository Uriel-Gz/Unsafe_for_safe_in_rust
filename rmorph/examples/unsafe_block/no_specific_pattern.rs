// Pattern: unsafe_block (sin patrones específicos detectables dentro)
fn main() {
    unsafe {
        // Operación que no genera detecciones específicas (solo unsafe block genérico)
        println!("unsafe_block -> solo println dentro de unsafe");
    }
}
