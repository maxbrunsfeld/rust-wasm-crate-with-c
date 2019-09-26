extern "C" {
    #[no_mangle]
    fn increment_in_c(i: u32) -> u32;
}

#[no_mangle]
extern "C" fn increment_in_rust(i: u32) -> u32 {
    unsafe { increment_in_c(i) }
}
