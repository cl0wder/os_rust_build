#![no_std]
#![no_main]
//#![warn(dead_code)]
use core::panic::PanicInfo;
//use std::panic;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //write!(DEBUG_OUTPUT, "panicked: {}", panic_info.message());
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
