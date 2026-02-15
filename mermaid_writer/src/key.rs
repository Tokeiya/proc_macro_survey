use super::error;
use crate::prelude::*;
use std::hash::Hash;

pub trait Key: Eq + Hash + Clone + Render {}

#[cfg(test)]
pub(crate) mod test_helper {
	use super::error::Result;
	use crate::key::Key;
	use crate::prelude::Render;
	use std::io::Write;

	impl Render for i32 {
		fn render(&self, write: &mut dyn Write) -> Result<(), ()> {
			write!(write, "{}", self)?;
			Ok(())
		}
	}

	impl Key for i32 {}

	impl Render for String {
		fn render(&self, write: &mut dyn Write) -> Result<(), ()> {
			write!(write, "{}", self)?;
			Ok(())
		}
	}

	impl Key for String {}
}
