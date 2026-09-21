use std::mem::MaybeUninit;

fn x() -> *mut MaybeUninit<u16> {
    Box::into_raw(Box::new(MaybeUninit::uninit()))
}

fn main() {
    let mut v = Box::new(10);
    let p = &mut *v as *mut i32;
    unsafe {
        let _s = 2 + 2;
        *p = 42;
    }

    let raw_v = &mut *v as *mut i32;
    unsafe {
        let _t = 2 + 2;
        let _ = *raw_v;
    }

    unsafe {
        let c = x();
        let _ = Some(*c);
        drop(Box::from_raw(c));
    }
}
