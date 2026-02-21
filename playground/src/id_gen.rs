use mermaid_writer::prelude::{Key, Render};
use std::hash::Hash;
use std::io::Write;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Integer(usize);

impl Render for Integer {
	fn render(&self, write: &mut dyn Write) -> mermaid_writer::error::Result<(), ()> {
		write!(write, "{}", self.0)?;
		Ok(())
	}
}

impl Key for Integer {}

pub struct IdGen(Integer);

impl Default for IdGen {
	fn default() -> Self {
		IdGen(Integer(0))
	}
}

impl IdGen {
	pub fn next(&mut self) -> Integer {
		self.0.0 += 1;
		self.0
	}

	pub fn current(&self) -> Integer {
		self.0
	}
}
