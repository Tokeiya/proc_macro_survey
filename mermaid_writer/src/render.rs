use crate::prelude::*;
use std::io::Write as IoWrite;
pub trait Render {
	fn render(&self, writer: &mut dyn IoWrite) -> Result<()>;
}

#[cfg(test)]
pub mod test_helper {
	use crate::prelude::Render;

	pub fn assert(actual: &[u8], expected: &str) {
		assert_eq!(actual, expected.as_bytes())
	}

	pub fn assert_render(actual: &dyn Render, expected: &str) {
		let mut write: Vec<u8> = Vec::new();

		actual.render(&mut write).unwrap();
		assert_eq!(write, expected.as_bytes());
	}
}
