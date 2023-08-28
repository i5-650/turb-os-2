#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(turb_os_2::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod drivers;
mod utils;


#[no_mangle]
pub extern "C" fn _start() -> ! {
	println!("Hello, 2+2={}", 2+2);

	#[cfg(test)]
	test_main();
	
	loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	turb_os_2::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
	assert_eq!(1, 1);
}