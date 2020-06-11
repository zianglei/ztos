#![feature(global_asm)]

global_asm!(include_str!("boot/entry64.asm"));

use crate::io;
use crate::sbi;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn _start();
        fn bootstacktop();
    }
    println!("_start vaddr = 0x{:x}", _start as usize);
    println!("bootstacktop vaddr = 0x{:x}", bootstacktop as usize);
    println!("hello world!");
    panic!("you want to nothing!");
    loop {}
}


