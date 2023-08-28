#![no_std]
#![no_main]


use turb_os_2::*;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	should_fail();
	serial_println!("[test did not panic]");
	exit_qemu(QemuExitCode::Failed);
	loop {}
}

fn should_fail() {
	println!("In should_fail");
	turb_os_2::serial_print!("should_panic::should_fail...\t");
	assert_eq!(0, 1);
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	turb_os_2::serial_println!("[ok]");
	exit_qemu(QemuExitCode::Success);
	loop {}
}