use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

static STANDARD_SERIAL_PORT: u16 = 0x3F8;

lazy_static! {
	pub static ref UART: Mutex<SerialPort> = {
		let mut serial_port = unsafe { SerialPort::new(STANDARD_SERIAL_PORT) };
		serial_port.init();
		Mutex::new(serial_port)
	};
}