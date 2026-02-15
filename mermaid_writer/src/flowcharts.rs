use super::error::Result;
use crate::error::Error;
use crate::link::Link;
use crate::node::Node;
use crate::orientations::Orientation;
use crate::prelude::*;
use crate::render::Render;
use std::collections::HashMap;
use std::io::Write;

//think about link duplication and node duplication.
pub struct Flowchart<K: Key> {
	title: String,
	orientation: Orientation,
	nodes: HashMap<K, Box<dyn Node<K>>>,
	links: HashMap<Connection<K>, Box<dyn Link<K>>>,
}

impl<K: Key> Flowchart<K> {
	pub fn new(title: String, orientation: Orientation) -> Self {
		Self {
			title,
			orientation,
			nodes: HashMap::new(),
			links: HashMap::new(),
		}
	}

	pub fn add_node<N: Node<K> + 'static>(&mut self, node: N) -> Result<K, K> {
		let key = node.id();
		if self.nodes.contains_key(&key) {
			Err(Error::NodeAlreadyExists(key))
		} else {
			assert!(self.nodes.insert(key.clone(), Box::new(node)).is_none());
			Ok(key)
		}
	}

	pub fn add_link<L: Link<K> + 'static>(&mut self, link: L) -> Result<Connection<K>, K> {
		if link.source() == link.target() {
			Err(Error::ScrTgtSameKey(link.source().clone()))
		} else if !self.nodes.contains_key(&link.source()) {
			Err(Error::KeyNotFound(link.source().clone()))
		} else if !self.nodes.contains_key(&link.target()) {
			Err(Error::KeyNotFound(link.target().clone()))
		} else {
			let con = Connection::from(&link);

			if self.links.contains_key(&con) {
				Err(Error::LinkAlreadyExists(con))
			} else {
				self.links.insert(con.clone(), Box::new(link));
				Ok(con)
			}
		}
	}

	pub fn nodes(&self) -> &HashMap<K, Box<dyn Node<K>>> {
		&self.nodes
	}

	pub fn links(&self) -> &HashMap<Connection<K>, Box<dyn Link<K>>> {
		&self.links
	}

	pub fn title(&self) -> &str {
		&self.title
	}

	pub fn write<W: Write>(&self, mut write: W) -> Result<(), ()> {
		writeln!(&mut write, "---")?;
		writeln!(&mut write, "title:{}", self.title())?;
		writeln!(&mut write, "---")?;

		let tmp = match self.orientation {
			Orientation::TopToBottom => "TB",
			Orientation::TopDown => "TD",
			Orientation::BottomToTop => "BT",
			Orientation::RightToLeft => "RL",
			Orientation::LeftToRight => "LR",
		};

		writeln!(&mut write, "flowchart {tmp}")?;

		for node in self.nodes.values() {
			write!(&mut write, "\t")?;
			node.render(&mut write)?;
			writeln!(&mut write)?;
		}

		for link in self.links.values() {
			write!(&mut write, "\t")?;
			link.render(&mut write)?;
			writeln!(&mut write)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::regular_link::RegularLink;
	use crate::regular_node::RegularNode;
	use std::ptr;
	#[test]
	fn new() {
		let fixture = Flowchart::<i32>::new("title".to_string(), Orientation::BottomToTop);
		assert_eq!(fixture.orientation, Orientation::BottomToTop);
		assert_eq!(fixture.links.len(), 0);
		assert_eq!(fixture.nodes.len(), 0);
	}

	#[test]
	fn add_node() {
		let mut fixture = Flowchart::new("title".to_string(), Orientation::BottomToTop);
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
		let mut fixture = Flowchart::<i32>::new("title".to_string(), Orientation::TopDown);
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

		let act = fixture.add_link(RegularLink::invisible(42, 43));
		assert!(matches!(act,Err(Error::KeyNotFound(i))if i==43));

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
		assert!(matches!(act,Error::LinkAlreadyExists(con) if con.source==24 && con.target==42));
		assert_eq!(fixture.links.len(), 2);
	}

	#[test]
	fn links() {
		let fixture = Flowchart::<i32>::new("title".to_string(), Orientation::TopDown);
		assert!(ptr::eq(&fixture.links, fixture.links()));
	}

	#[test]
	fn nodes() {
		let fixture = Flowchart::<i32>::new("title".to_string(), Orientation::TopDown);
		assert!(ptr::eq(&fixture.nodes, fixture.nodes()));
	}

	#[test]
	fn orientation() {
		let fixture = Flowchart::<i32>::new("titi".to_string(), Orientation::TopDown);
		assert_eq!(fixture.orientation, Orientation::TopDown);
	}
	#[test]
	fn title() {
		let fixture = Flowchart::<i32>::new("title".to_string(), Orientation::TopDown);
		assert_eq!(fixture.title(), "title");
	}
}
