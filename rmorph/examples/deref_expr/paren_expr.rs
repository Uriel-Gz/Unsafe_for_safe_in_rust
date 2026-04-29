// Pattern: deref_expr (desreferenciación con paréntesis)
fn main() {
    let x = 7i32;
    let p = &x as *const i32;
    unsafe {
        let v = *({ p }); // paréntesis alrededor de la expresión
        println!("deref_expr paren -> {}", v);
    }
}
