use crate::prelude::Render;
use std::hash::Hash;

pub trait Key: Eq + Hash + Clone + Render {}

#[cfg(test)]
pub(super) mod test_helper {
	use crate::key::Key;
	use crate::prelude::Render;
	use std::io::Write;

	impl Render for i32 {
		fn render(&self, writer: &mut dyn Write) -> crate::error::Result<()> {
			write!(writer, "{}", self)?;
			Ok(())
		}
	}

	impl Key for i32 {}

	impl Render for String {
		fn render(&self, writer: &mut dyn Write) -> crate::error::Result<()> {
			write!(writer, "{}", self)?;
			Ok(())
		}
	}

	impl Key for String {}
}
