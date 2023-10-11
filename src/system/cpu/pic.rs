use pic8259::ChainedPics;
use spin;

pub const PIC_PRIMARY_OFFSET: u8 = 32;
pub const PIC_SECONDARY_OFFSET: u8 = PIC_PRIMARY_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
	unsafe { ChainedPics::new(PIC_PRIMARY_OFFSET, PIC_SECONDARY_OFFSET) }
);