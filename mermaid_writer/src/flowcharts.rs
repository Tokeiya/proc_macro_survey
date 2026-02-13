use super::error::{Error, Result};
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
	pub fn add_node<N: Node<K>>(&mut self, node: N) -> Result<K> {
		todo!()
	}

	pub fn add_link<L: Link<K>>(&mut self, link: K) -> Result<K> {
		todo!()
	}

	pub fn nodes(&self) -> &HashMap<K, Box<dyn Node<K>>> {
		&self.nodes
	}

	pub fn links(&self) -> &HashMap<Connection<K>, Box<dyn Link<K>>> {
		&self.links
	}
}

impl<K: Key> Default for Flowchart<K> {
	fn default() -> Self {
		todo!()
	}
}

impl<K: Key> Render for Flowchart<K> {
	fn render(&self, write: &mut dyn Write) -> Result<()> {
		todo!()
	}
}
