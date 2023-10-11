use core::fmt::{Write, Arguments, Result};
use crate::drivers::vga::vga_driver::*;

impl Write for VgaWriter {
	fn write_str(&mut self, s: &str) -> Result {
		self.write_str(s);
		return Ok(());
	}
}


#[macro_export]
macro_rules! print {
	($($arg:tt)*) => (crate::drivers::vga::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		let _ = WRITER.lock().write_fmt(args);
	});
}