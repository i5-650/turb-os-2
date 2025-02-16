#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(turb_os_2::test_runner)]
#![reexport_test_harness_main = "test_main"]

use turb_os_2::hlt_loop;

mod drivers;
mod utils;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome on Turb-OS-2");

    turb_os_2::init();

    #[cfg(test)]
    test_main();

    println!("you made it boy !");

    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    turb_os_2::test_panic_handler(info)
}
