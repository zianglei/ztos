#![no_std]
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
   loop {}
}

#![feature(global_asm)]

global_asm!(include_str!("boot/entry64.asm"));

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main() -> ! {
    loop {}
}

