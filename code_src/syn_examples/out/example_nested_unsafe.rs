fn main() {
    let x = 1;
    unsafe {
        let y = { unsafe { x + 1 } };
        println!("y = {}", y);
    }
}
