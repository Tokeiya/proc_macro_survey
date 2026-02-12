use crate::contents_format::Format;
use crate::key::Key;
use crate::node::Node;
use crate::node_shape::Shape;
use crate::prelude::Render;
use std::hash::Hash;
use std::io::Write;

pub struct RegularNode<K> {
	key: K,
	shape: Shape,
	format: Format,
	contents: Option<String>,
}

impl<K: Key> RegularNode<K> {
	pub fn new(key: K, shape: Shape, format: Format, contents: Option<String>) -> Self {
		Self {
			key,
			shape,
			format,
			contents,
		}
	}

	pub fn textless_new(key: K, shape: Shape) -> Self {
		Self {
			key,
			shape,
			format: Format::Text,
			contents: None,
		}
	}
}

impl<K: PartialEq + Hash + Clone> Render for RegularNode<K> {
	fn render(&self, writer: &mut dyn Write) -> crate::error::Result<()> {
		todo!()
	}
}

impl<K: Key> Node<K> for RegularNode<K> {
	fn id(&self) -> K {
		self.key.clone()
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
	use crate::render::test_helper::assert_render;

	#[test]
	fn new() {
		let fixture = RegularNode::new(
			1,
			Shape::Rect,
			Format::Text,
			Some("Hello world".to_string()),
		);
		assert_eq!(fixture.key, 1);
		assert_eq!(fixture.shape, Shape::Rect);
		assert_eq!(fixture.format, Format::Text);
		assert_eq!(fixture.contents.unwrap(), "Hello world");
	}

	#[test]
	fn textless_new() {
		let fixture = RegularNode::textless_new(1, Shape::Rect);
		assert_eq!(fixture.key, 1);
		assert_eq!(fixture.shape, Shape::Rect);
		assert_eq!(fixture.contents, None);
	}

	#[test]
	fn id() {
		let fixture = RegularNode::textless_new(1, Shape::Rect);
		assert_eq!(fixture.id(), 1);

		let fixture = RegularNode::new(
			1,
			Shape::Rect,
			Format::Text,
			Some("Hello world".to_string()),
		);
		assert_eq!(fixture.id(), 1)
	}

	#[test]
	fn format() {
		let fixture = RegularNode::new(
			1,
			Shape::Rect,
			Format::Text,
			Some("Hello world".to_string()),
		);
		assert_eq!(fixture.format(), Format::Text);

		let fixture = RegularNode::new(
			1,
			Shape::Rect,
			Format::Markdown,
			Some("**MarkDown**".to_string()),
		);
		assert_eq!(fixture.format(), Format::Markdown);

		let fixture = RegularNode::textless_new(1, Shape::Rect);
		assert_eq!(fixture.format(), Format::Text)
	}

	#[test]
	fn contents() {
		let fixture = RegularNode::new(
			1,
			Shape::Rect,
			Format::Text,
			Some("Hello world".to_string()),
		);
		assert_eq!(fixture.contents(), Some("Hello world"));

		let fixture = RegularNode::textless_new(1, Shape::Rect);
		assert_eq!(fixture.contents(), None);
	}

	#[test]
	fn render() {
		let fixture = RegularNode::new(
			1,
			Shape::Rect,
			Format::Text,
			Some("Hello world".to_string()),
		);
		assert_render(&fixture, "1[\"Hello world\"]");

		let fixture = RegularNode::new(
			"Hello".to_string(),
			Shape::Rounded,
			Format::Markdown,
			Some("Hello world".to_string()),
		);
		assert_render(&fixture, "Hello(\"`Hello world`\")");
	}
}
