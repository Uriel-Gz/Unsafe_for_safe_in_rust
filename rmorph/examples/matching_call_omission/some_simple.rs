// Pattern: matching_call_omission (llamada a `Some(...)` dentro de unsafe)
fn main() {
    unsafe {
         let c = 2 + 2;
        let _ = Some(*c);
    } // detector busca llamadas a `Some` dentro de unsafe
}
