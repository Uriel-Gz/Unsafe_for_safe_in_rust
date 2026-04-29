// Pattern: matching_call_omission (llamada a `Some(...)` dentro de unsafe)
fn main() {
    unsafe {
        let _ = Some(42); // detector busca llamadas a `Some` dentro de unsafe
        println!("matching_call_omission -> Some dentro de unsafe");
    }
}
