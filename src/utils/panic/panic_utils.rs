use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { // ! = "never" type
	println!("{}", _info);
	loop {}
}
