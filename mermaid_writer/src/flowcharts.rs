use crate::link::Link;
use crate::node::Node;
use crate::orientations::Orientation;
use crate::prelude::Key;
use std::collections::HashMap;
use std::hash::Hash;
#[derive(PartialEq, Eq, Hash, Clone)]
struct Connection<K: Key> {
	pub source: K,
	pub target: K,
}

//think about link duplication and node duplication.
pub struct Flowchart<K: Key> {
	orientation: Orientation,
	nodes: HashMap<K, Box<dyn Node<K>>>,
	links: HashMap<Connection<K>, Box<dyn Link<K>>>,
}
