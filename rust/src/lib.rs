#![no_std]

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn get_rust_u8() -> u8 {
    //9
    unsafe { get_c_u8() }
}

extern "C" {
    fn get_c_u8() -> u8;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(get_rust_u8(), 9);
    }
}
