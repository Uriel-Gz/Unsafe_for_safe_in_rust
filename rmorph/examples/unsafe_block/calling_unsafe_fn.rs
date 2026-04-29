// Pattern: unsafe_block (contiene llamada a función unsafe dentro de unsafe)
unsafe fn danger() { println!("danger executed"); }

fn main() {
    unsafe {
        // llamada a función marcada como unsafe (seguirá siendo parte del bloque)
        danger();
        println!("unsafe_block -> llamada a unsafe dentro del bloque");
    }
}
