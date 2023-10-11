use crate::pic;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
	Timer = pic::PIC_PRIMARY_OFFSET
}

impl InterruptIndex {
	pub fn as_u8(self) -> u8 {
		return self as u8
	}

	pub fn as_usize(self) -> usize {
		return usize::from(self.as_u8())
	}
}