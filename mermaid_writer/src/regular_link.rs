use crate::contents_format::Format;
use crate::direction::Direction;
use crate::edge_shape::Shape;
use crate::prelude::*;
use std::hash::Hash;
use std::io::Write;

pub struct RegularLink<K> {
	source: K,
	target: K,
	shape: Shape,
	direction: Direction,
	format: Format,
	contents: Option<String>,
}

impl<K: Key> RegularLink<K> {
	pub fn new(
		source: K,
		target: K,
		shape: Shape,
		direction: Direction,
		format: Format,
		contents: Option<String>,
	) -> Self {
		Self {
			source,
			target,
			shape,
			direction,
			format,
			contents,
		}
	}

	pub fn textless_new(source: K, target: K, shape: Shape, direction: Direction) -> Self {
		Self {
			source,
			target,
			shape,
			direction,
			format: Format::Text,
			contents: None,
		}
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

	fn format(&self) -> Format {
		self.format.clone()
	}

	fn contents(&self) -> Option<&str> {
		self.contents.as_deref()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_prelude::*;

	#[test]
	fn new() {
		let link: RegularLink<i32> = RegularLink::new(
			1,
			2,
			Shape::Normal,
			Direction::Both,
			Format::Markdown,
			Some("Link Contents".to_string()),
		);
		assert_eq!(link.source, 1);
		assert_eq!(link.target, 2);
		assert_eq!(link.shape, Shape::Normal);
		assert_eq!(link.direction, Direction::Both);
		assert_eq!(link.format, Format::Markdown);
		assert_eq!(link.contents.unwrap(), "Link Contents");
	}

	#[test]
	fn textless_new() {
		let link: RegularLink<i32> =
			RegularLink::textless_new(1, 2, Shape::Dotted, Direction::Both);
		assert_eq!(link.source, 1);
		assert_eq!(link.target, 2);
		assert_eq!(link.shape, Shape::Dotted);
		assert_eq!(link.direction, Direction::Both);
		assert_eq!(link.format, Format::Text);
		assert_eq!(link.contents, None);
	}

	#[test]
	fn source() {
		let link: RegularLink<i32> =
			RegularLink::textless_new(1, 2, Shape::Normal, Direction::Oneway);
		assert_eq!(link.source(), 1);
	}

	#[test]
	fn target() {
		let link: RegularLink<i32> =
			RegularLink::textless_new(1, 2, Shape::Normal, Direction::Oneway);
		assert_eq!(link.target(), 2);
	}

	#[test]
	fn direction() {
		let link: RegularLink<i32> =
			RegularLink::textless_new(1, 2, Shape::Normal, Direction::Oneway);
		assert_eq!(link.direction(), Direction::Oneway);
	}

	#[test]
	fn format() {
		let link: RegularLink<i32> =
			RegularLink::textless_new(1, 2, Shape::Normal, Direction::Oneway);
		assert_eq!(link.format(), Format::Text);

		let link = RegularLink::new(
			1,
			2,
			Shape::Normal,
			Direction::Oneway,
			Format::Markdown,
			Some("Contents".to_string()),
		);
		assert_eq!(link.format(), Format::Markdown);
	}

	#[test]
	fn contents() {
		let link: RegularLink<i32> = RegularLink::new(
			1,
			2,
			Shape::Normal,
			Direction::Oneway,
			Format::Markdown,
			Some("Contents".to_string()),
		);
		assert_eq!(link.contents(), Some("Contents"));

		let link: RegularLink<i32> =
			RegularLink::textless_new(1, 2, Shape::Normal, Direction::Oneway);
		assert_eq!(link.contents(), None);
	}
}
