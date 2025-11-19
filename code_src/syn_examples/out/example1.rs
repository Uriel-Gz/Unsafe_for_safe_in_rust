fn main() {
    let x = 5;
    unsafe {
        let p = &x as *const i32;
        println!("{:p}", p);
    }
    let y = unsafe { *(&x as *const i32) };
    unsafe {
        let z = y + 10;
        println!("z = {}", z);
    }
    unsafe {
        let arr = [1, 2, 3, 4, 5];
        let ptr = arr.as_ptr().add(2);
        println!("Third element: {}", * ptr);
    }
    println!("y = {}", y);
}
