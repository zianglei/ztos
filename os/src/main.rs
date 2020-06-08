
#![no_std]
#![no_main] // disable all Rust-level entry points
#![feature(asm)]

#![feature(global_asm)]

global_asm!(include_str!("boot/entry64.asm"));



use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
   loop {}
}

pub fn console_putchar(ch: u8) {
    let ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile"
            );
    }
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main() -> ! {
    console_putchar(b'0');
    console_putchar(b'K');
    console_putchar(b'\n');
    loop {}
}

