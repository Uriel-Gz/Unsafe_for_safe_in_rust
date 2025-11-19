extern "C" {
    fn puts(s: *const i8) -> i32;
}
fn main() {
    unsafe {
        let s = b"hello\0".as_ptr() as *const i8;
        puts(s);
    }
}
