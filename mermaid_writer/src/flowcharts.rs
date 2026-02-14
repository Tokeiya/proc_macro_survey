use super::error::Result;
use crate::link::Link;
use crate::node::Node;
use crate::orientations::Orientation;
use crate::prelude::Key;
use crate::render::Render;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Connection<K: Key> {
	pub source: K,
	pub target: K,
}

//think about link duplication and node duplication.
pub struct Flowchart<K: Key> {
	orientation: Orientation,
	nodes: HashMap<K, Box<dyn Node<K>>>,
	links: HashMap<Connection<K>, Box<dyn Link<K>>>,
}

impl<K: Key> Flowchart<K> {
	pub fn new(orientation: Orientation) -> Self {
		_ = orientation.clone();
		todo!()
	}

	pub fn add_node<N: Node<K>>(&mut self, node: N) -> Result<K, K> {
		_ = node.format();
		todo!()
	}

	pub fn add_link<L: Link<K>>(&mut self, link: K) -> Result<K, K> {
		_ = link.clone();
		todo!()
	}

	pub fn nodes(&self) -> &HashMap<K, Box<dyn Node<K>>> {
		&self.nodes
	}

	pub fn links(&self) -> &HashMap<Connection<K>, Box<dyn Link<K>>> {
		&self.links
	}
}

impl<K: Key> Render for Flowchart<K> {
	fn render(&self, write: &mut dyn Write) -> Result<(), ()> {
		_ = write.flush();
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::key::test_helper;
	use crate::line_style::{Shape as LineShape, Style};
	use crate::node_shape::Shape as NodeShape;
	use crate::prelude::*;
	use crate::regular_link::RegularLink;
	use crate::regular_node::RegularNode;
	#[test]
	fn new() {
		let fixture = Flowchart::<i32>::new(Orientation::BottomToTop);
		assert_eq!(fixture.orientation, Orientation::BottomToTop);
		assert_eq!(fixture.links.len(), 0);
		assert_eq!(fixture.nodes.len(), 0);
	}

	#[test]
	fn add_node() {
		let mut fixture = Flowchart::new(Orientation::BottomToTop);
		let act = fixture
			.add_node(RegularNode::new(
				42,
				NodeShape::Circle,
				Some("Circle".to_string()),
			))
			.unwrap();
		assert_eq!(act, 42);
		assert_eq!(fixture.nodes.len(), 1);
		let act = fixture.nodes.get(&42).unwrap();
		assert_eq!(act.id(), 42);
		assert_eq!(act.shape(), NodeShape::Circle);

		let err = fixture.add_node(RegularNode::new(
			42,
			NodeShape::Rect,
			Some("Rect".to_string()),
		));
		todo!()
	}
}
