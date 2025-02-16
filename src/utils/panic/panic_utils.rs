use crate::println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // ! = "never" type
    println!("{info}");
    loop {}
}
