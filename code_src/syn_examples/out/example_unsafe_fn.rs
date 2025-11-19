unsafe fn dangerous() {
    let a = 10 as *const i32;
    let _ = unsafe { *a };
}
fn main() {
    unsafe { dangerous() }
}
