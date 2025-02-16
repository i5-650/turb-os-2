#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(turb_os_2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use turb_os_2::*;

#[cfg(test)]
#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    turb_os_2::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[test_case]
fn test_print() {
    print!("test_print outpout");
}

#[test_case]
fn test_print_format() {
    let a = 12.3;
    println!("test_print_format {}", a);
}
