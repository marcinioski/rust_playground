fn call() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;
}

unsafe fn call1() {
}

mod ffi;

fn main() {
    println!("Hello, world!");
    call();

    unsafe {
        call1();
        ffi::call_abs(10);
    }
}
