use crate::arrow_shape::Shape as ArrowShape;
use crate::contents_format::Format;
use crate::direction::Direction;
use crate::line_shape::Shape as LineShape;
use crate::prelude::*;
use std::hash::Hash;
use std::io::Write;

pub struct RegularLink<K> {
	source: K,
	target: K,
	line_shape: LineShape,
	arrow_shape: Option<ArrowShape>,
	direction: Direction,
	format: Format,
	contents: Option<String>,
}

impl<K: Key> RegularLink<K> {
	pub fn try_new(
		source: K,
		target: K,
		line_shape: LineShape,
		arrow_shape: Option<ArrowShape>,
		direction: Direction,
		format: Format,
		contents: Option<String>,
	) -> Result<Self> {
		todo!()
	}

	pub fn textless_new(
		source: K,
		target: K,
		line_shape: LineShape,
		arrow_shape: Option<ArrowShape>,
		direction: Direction,
	) -> Result<Self> {
		todo!()
	}
}

impl<K: PartialEq + Hash + Clone> Render for RegularLink<K> {
	fn render(&self, write: &mut dyn Write) -> Result<()> {
		todo!()
	}
}

impl<K: Key> Link<K> for RegularLink<K> {
	fn source(&self) -> K {
		self.source.clone()
	}

	fn target(&self) -> K {
		self.target.clone()
	}

	fn direction(&self) -> Direction {
		self.direction
	}

	fn line_shape(&self) -> LineShape {
		self.line_shape.clone()
	}

	fn arrow_shape(&self) -> Option<ArrowShape> {
		self.arrow_shape.clone()
	}

	fn format(&self) -> Format {
		self.format.clone()
	}

	fn contents(&self) -> Option<&str> {
		self.contents.as_deref()
	}
}
