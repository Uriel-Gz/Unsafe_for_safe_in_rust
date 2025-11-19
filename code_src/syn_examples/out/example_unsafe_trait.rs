trait MyTrait {
    fn safe_method(&self);
}
unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}
struct S;
impl MyTrait for S {
    fn safe_method(&self) {
        println!("safe");
    }
}
unsafe impl UnsafeTrait for S {
    fn unsafe_method(&self) {
        println!("unsafe trait method");
    }
}
fn main() {
    unsafe {
        let s = S;
        s.unsafe_method();
    }
}
