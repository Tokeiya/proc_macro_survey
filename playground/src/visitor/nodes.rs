use super::id::Id;
use mermaid_writer::node_shape::Shape;
use mermaid_writer::prelude::{self, *};
use mermaid_writer::regular_node::RegularNode;
use std::io::Write;

pub struct Node(RegularNode<Id>);

impl Render for Node {
	fn render(&self, write: &mut dyn Write) -> error::Result<(), ()> {
		self.0.render(write)
	}
}

impl Node {
	pub fn from_str(id: Id, contents: &str) -> Self {
		Self(RegularNode::new(
			id,
			Shape::Rect,
			Some(contents.to_string()),
		))
	}
}

impl prelude::Node<Id> for Node {
	fn id(&self) -> Id {
		self.0.id()
	}

	fn shape(&self) -> NodeShape {
		self.0.shape()
	}

	fn format(&self) -> ContentsFormat {
		self.0.format()
	}

	fn contents(&self) -> Option<&str> {
		self.0.contents()
	}
}
