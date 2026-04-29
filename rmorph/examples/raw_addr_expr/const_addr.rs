// Pattern: raw_addr_expr (&raw const ...)
fn main() {
    let v = 10i32;
    let r = &raw const v;
    // r es un puntero crudo creado con `&raw const` -> `raw_addr_expr`
    unsafe { println!("raw_addr_expr const -> {:p}", r); }
}
