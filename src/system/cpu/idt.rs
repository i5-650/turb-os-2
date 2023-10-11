use crate::pic::PICS;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use crate::print;
use crate::println;
use crate::gdt;
use crate::pic;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = pic::PIC_PRIMARY_OFFSET,
	Keyboard
}

impl InterruptIndex {
	pub fn as_u8(self) -> u8 {
		return self as u8
	}

	pub fn as_usize(self) -> usize {
		return usize::from(self.as_u8())
	}
}



lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);

		unsafe {
			idt.double_fault
				.set_handler_fn(double_fault_handler)
				.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
		}

		idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
		idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

		return idt;
	};
}

pub fn init() {
	IDT.load();
}


lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
        	ScancodeSet1::new(),
        	layouts::Us104Key, 
            HandleControl::Ignore)
        );
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("[EXCEPTION] Breakpoint\n {:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
	panic!("[EXCEPTION] Double Fault:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
	//print!(".");

	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
	use x86_64::instructions::port::Port;
	
	let mut port = Port::new(0x60);
	let mut keyboard = KEYBOARD.lock();

	let scancode: u8 = unsafe { port.read() };

	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key {
			    DecodedKey::Unicode(chara) => print!("{}", chara),
			    DecodedKey::RawKey(key) => print!("{:?}", key)
			}
		}
	}


	unsafe {
		PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
	}
}

#[test_case]
fn breakpoint_exception_test() {
	x86_64::instructions::interrupts::int3();
}