use crate::drivers::vga::vga_driver::{VgaWriter, WRITER};
use core::fmt::{Arguments, Result, Write};

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga::macros::printer(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn printer(args: Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
