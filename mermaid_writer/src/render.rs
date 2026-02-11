use crate::prelude::*;
use std::io::Write as IoWrite;
pub trait Render {
	fn write(&self, writer: &mut dyn IoWrite) -> Result<()>;
}

#[cfg(test)]
pub mod test_helper {
	pub fn assert(actual: &[u8], expected: &str) {
		assert_eq!(actual, expected.as_bytes())
	}
}
