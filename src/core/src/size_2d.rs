#[derive(Default, Debug, Copy, Clone)]
pub struct Size2dU32 {
	pub width: u32,
	pub height: u32
}

impl Size2dU32 {
	pub fn is_zero(&self) -> bool {
		return self.width == 0 && self.height == 0
	}
}
