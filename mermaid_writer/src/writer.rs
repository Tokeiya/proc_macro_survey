use crate::prelude::*;
use std::io::Write as IoWrite;
pub trait Writer<W: IoWrite> {
	fn write(&self, writer: &mut W) -> Result<()>;
}

#[cfg(test)]
pub mod test_helper {
	pub fn assert(actual: &[u8], expected: &str) {
		assert_eq!(actual, expected.as_bytes())
	}
}
