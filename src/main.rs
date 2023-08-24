#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // ! = "never type"
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	let vga_buffer = 0xb8000 as *mut u8;

	// for each letter of HELLO string
	for (i, &byte) in HELLO.iter().enumerate() {
		// unsafe because the compiler cannot be sure the raw pointer is valid
		// so by saying unsafe, we tell the compiler that we (coder) are sure it's valid
		unsafe {
			// the letter byte 
			*vga_buffer.offset(i as isize * 2) = byte;
			// the color byte
			*vga_buffer.offset(i as isize * 2 + 1) = 0xf; // 0xf = white
		}
	}

	loop {}
}