extern "C" {
    fn abs(input: i32) -> i32;
}

// permit compiler before changing the signature name of the function
#[no_mangle]
pub unsafe fn call_abs(i: i32) -> i32 {
    abs(i)
}

#[no_mangle]
pub unsafe extern "C" fn call_from_outside() {
}
