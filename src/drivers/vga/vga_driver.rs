use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::drivers::vga::structs::{Color, BUFF_HEIGHT, BUFF_WIDTH, NEW_LINE, VGA_BUFF_ADDR};

lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
        column: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buff: unsafe { &mut *(VGA_BUFF_ADDR as *mut Buffer) }
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // because the 3 low bits are for background, 4 high for foreground
        ColorCode((background as u8) << 4 | foreground as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // layout exactly like in C
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

pub struct VgaWriter {
    column: usize,
    color_code: ColorCode,
    // to make the reference lifetime last for the whole program run time
    buff: &'static mut Buffer,
}

impl VgaWriter {
    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFF_WIDTH {
            self.buff.chars[row][col].write(ScreenChar {
                ascii_char: b' ',
                color_code: self.color_code,
            });
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFF_HEIGHT {
            for col in 0..BUFF_WIDTH {
                let ch = self.buff.chars[row][col].read();
                self.buff.chars[row - 1][col].write(ch);
            }
        }

        self.clear_row(BUFF_HEIGHT - 1);
        self.column = 0;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            NEW_LINE => self.new_line(),

            byte => {
                if self.column >= BUFF_WIDTH {
                    self.new_line();
                }

                self.buff.chars[BUFF_HEIGHT - 1][self.column].write(ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                });
                self.column += 1;
            }
        }
    }

    pub fn write_str(&mut self, stri: &str) {
        for byte in stri.bytes() {
            match byte {
                // 0x20 = space aka 1st printable char
                // 0x7e = 126 aka before last char in ascii table
                0x20..=0x7e | NEW_LINE => self.write_byte(byte),
                _ => self.write_byte(0xfe), // white block
            }
        }
    }
}

#[repr(transparent)] // layout as its single field
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFF_WIDTH]; BUFF_HEIGHT],
}

