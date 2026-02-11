use super::error::Result;
use crate::link::Link;
use crate::node::Node;
use crate::orientations::Orientation;
use crate::prelude::Render;
use std::hash::Hash;
use std::io::Write;

pub struct Flowchart<K: PartialEq + Hash + Clone> {
	orientation: Orientation,
	nodes: Vec<Box<dyn Node<K>>>,
	links: Vec<Box<dyn Link<K>>>,
}
