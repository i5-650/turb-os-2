use crate::pic::PICS;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use crate::print;
use crate::println;
use crate::gdt;
use crate::system::cpu::timer::InterruptIndex;



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

		return idt;
	};
}

pub fn init() {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
	println!("[EXCEPTION] Breakpoint\n {:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
	panic!("[EXCEPTION] Double Fault:\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
	print!(".");

	unsafe {
		PICS.lock()
			.notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
	}
}

#[test_case]
fn breakpoint_exception_test() {
	x86_64::instructions::interrupts::int3();
}