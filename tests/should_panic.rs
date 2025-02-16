#![no_std]
#![no_main]

use core::panic::PanicInfo;
use turb_os_2::{exit_qemu, QemuExitCode};
use turb_os_2::{serial_print, serial_println};

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
