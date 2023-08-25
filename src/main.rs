#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod drivers;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // ! = "never" type
	println!("{}", _info);
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello, 2+2={}", 4);
	loop {}
}