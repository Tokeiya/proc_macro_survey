use mermaid_writer::key::Key;
use mermaid_writer::prelude::Render;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(i32);

impl From<i32> for Id {
	fn from(value: i32) -> Self {
		Self(value)
	}
}

impl Id {
	pub fn inclement(&mut self) -> Id {
		self.0 += 1;
		Id::from(self.0)
	}

	pub fn current(&self) -> i32 {
		self.0
	}
}

impl Render for Id {
	fn render(&self, write: &mut dyn Write) -> mermaid_writer::error::Result<(), ()> {
		write!(write, "{}", self.0)?;
		Ok(())
	}
}

impl Key for Id {}
