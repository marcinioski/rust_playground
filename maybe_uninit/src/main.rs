use std::mem::{self, MaybeUninit};

const ARR_SIZE: usize = 10;

//fn call<T>(x: T) {
//    let x_ptr: *const u32 = &x;
//}
//

fn call1(x: u32) {
    let x_ptr: *const u32 = &x;
    println!("x_ptr: {:x}", (x_ptr as u32));
}

fn call2(x: &u32) {
    let x_ptr: *const u32 = x;
    println!("x_ptr: {:x}", (x_ptr as u32));
}


fn main() {
    let x = {
        let mut x: [MaybeUninit<Box<u32>>; ARR_SIZE] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        for i in 0..ARR_SIZE {
            x[i] = MaybeUninit::new(Box::new(i as u32)); 
        }

        unsafe { mem::transmute::<_, [Box<u32>; ARR_SIZE]>(x) }
    };

    let x: u32 = 0xffff_ffff;
    let x_ptr: *const u32 = &x;
    println!("x_ptr: {:x}", (x_ptr as u32));

    call2(&x);
    call1(x);
}
