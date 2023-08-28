use crate::drivers::serial::serial_driver::UART;

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
	use core::fmt::Write;
	UART.lock()
		.write_fmt(args)
		.expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => {
		crate::drivers::serial::macros::_print(format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! serial_println {
	() => ($crate::serial_print!("\n"));
	($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
		concat!($fmt, "\n"), $($arg)*));
}