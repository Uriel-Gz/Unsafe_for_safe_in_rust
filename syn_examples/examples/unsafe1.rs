use std::mem::MaybeUninit;

fn x() -> *mut [MaybeUninit<u16>] {
    unsafe {
        let mut x = MaybeUninit::<[u16; 5]>::uninit();
        let raw_ptr = x.as_mut_ptr() as *mut MaybeUninit<u16>;
        let slice = std::slice::from_raw_parts_mut(raw_ptr, 5);
        &mut *(slice as *mut [MaybeUninit<u16>])
    }
}

fn main() {
    let mut v = 10;
    let p = v.as_mut_ptr();
    unsafe {
        *p.add(1) = 42;
    }

    unsafe {
        *v;
    }

    let c = x();

}
