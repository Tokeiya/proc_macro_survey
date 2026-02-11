use super::error::Result;
use crate::link::Link;
use crate::node::Node;
use crate::orientations::Orientation;
use crate::prelude::Writer;
use std::hash::Hash;
use std::io::Write;

pub struct Flowchart<K: PartialEq + Hash + Clone + Copy> {
	orientation: Orientation,
	nodes: Vec<Box<dyn Node<K>>>,
	links: Vec<Box<dyn Link<K>>>,
}
// impl<W: Write, K: PartialEq + Hash> Flowchart<K> {
// 	pub fn new(orientation: Orientation) -> Self {
// 		todo!()
// 	}
//
// 	pub fn add_node(&mut self, node: Box<dyn Node<K>>) -> Result<&K> {
// 		todo!()
// 	}
//
// 	pub fn add_link(&mut self, source: &K, target: &K) -> Result<()> {
// 		todo!()
// 	}
// }
//
// impl<K: PartialEq + Hash> Writer for Flowchart<K> {
// 	fn write(&self, writer: &mut W) -> crate::error::Result<()> {
// 		todo!()
// 	}
// }
