// Pattern: matching_call_omission (llamada con ruta que contiene `Some`)
fn main() {
    unsafe {
        // ruta completa que contiene el segmento `Some`
        let _ = ::std::option::Option::Some(100);
        println!("matching_call_omission -> Option::Some dentro de unsafe");
    }
}
