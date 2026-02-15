use super::error::Result;
use crate::link::Link;
use crate::node::Node;
use crate::orientations::Orientation;
use crate::prelude::*;
use crate::render::Render;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
		Self {
			orientation,
			nodes: HashMap::new(),
			links: HashMap::new(),
		}
	}

	pub fn add_node<N: Node<K> + 'static>(&mut self, node: N) -> Result<K, K> {
		let key = node.id();
		if self.nodes.contains_key(&key) {
			Err(Error::KeyNotFound(key))
		} else {
			assert!(self.nodes.insert(key.clone(), Box::new(node)).is_none());
			Ok(key)
		}
	}

	pub fn add_link<L: Link<K>>(&mut self, link: L) -> Result<Connection<K>, K> {
		_ = link;
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
	use crate::prelude::*;
	use crate::regular_link::RegularLink;
	use crate::regular_node::RegularNode;
	use std::ptr;
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
		let err = match err {
			Ok(_) => unreachable!(),
			Err(e) => e,
		};

		assert!(matches!(err, Error::NodeAlreadyExists(i) if i==42));
	}

	#[test]
	fn add_link() {
		let mut fixture = Flowchart::<i32>::new(Orientation::TopDown);
		let link = RegularLink::oneway(10, 20, LineShape::Dotted, ArrowShape::Arrow, None);
		let act = fixture.add_link(link);
		assert!(matches!(act,Err(Error::KeyNotFound(k)) if k==10));
		assert_eq!(fixture.links.len(), 0);

		_ = fixture.add_node(RegularNode::new(42, NodeShape::Diamond, None));
		let act = fixture.add_link(RegularLink::oneway(
			42,
			42,
			LineShape::Dotted,
			ArrowShape::Arrow,
			None,
		));
		assert!(matches!(act,Err(Error::ScrTgtSameKey(i))if i==42));
		assert_eq!(fixture.links.len(), 0);

		_ = fixture
			.add_node(RegularNode::new(24, NodeShape::Hex, None))
			.unwrap();

		let act = fixture.add_link(RegularLink::oneway(
			24,
			24,
			LineShape::Normal,
			ArrowShape::Arrow,
			None,
		));
		assert!(matches!(act,Err(Error::ScrTgtSameKey(i))if i==24));
		assert_eq!(fixture.links.len(), 0);

		let act = fixture
			.add_link(RegularLink::oneway(
				42,
				24,
				LineShape::Normal,
				ArrowShape::Arrow,
				None,
			))
			.unwrap();
		assert_eq!(act.source, 42);
		assert_eq!(act.target, 24);
		debug_assert_eq!(fixture.links.len(), 1);

		let act = fixture
			.add_link(RegularLink::oneway(
				24,
				42,
				LineShape::Dotted,
				ArrowShape::Circle,
				None,
			))
			.unwrap();
		assert_eq!(act.source, 24);
		assert_eq!(act.target, 42);
		assert_eq!(fixture.links.len(), 2);

		let act = fixture
			.add_link(RegularLink::oneway(
				24,
				42,
				LineShape::Dotted,
				ArrowShape::Circle,
				None,
			))
			.unwrap_err();
		assert!(matches!(act,Error::LinkAlreadyExists(i,j) if i==24 && j==42));
		assert_eq!(fixture.links.len(), 2);
	}

	#[test]
	fn links() {
		let fixture = Flowchart::<i32>::new(Orientation::TopDown);
		assert!(ptr::eq(&fixture.links, fixture.links()));
	}

	#[test]
	fn nodes() {
		let fixture = Flowchart::<i32>::new(Orientation::TopDown);
		assert!(ptr::eq(&fixture.nodes, fixture.nodes()));
	}

	#[test]
	fn orientation() {
		let fixture = Flowchart::<i32>::new(Orientation::TopDown);
		assert_eq!(fixture.orientation, Orientation::TopDown);
	}
}
