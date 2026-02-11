use crate::contents_format::Format;
use crate::node::Node;
use crate::node_shape::Shape;
use crate::prelude::Render;
use std::hash::Hash;
use std::io::Write;

pub struct RegularNode<K> {
	key: K,
	shape: Shape,
}

impl<K: PartialEq + Hash + Clone> Render for RegularNode<K> {
	fn write(&self, writer: &mut dyn Write) -> crate::error::Result<()> {
		todo!()
	}
}

impl<K: PartialEq + Hash + Clone> Node<K> for RegularNode<K> {
	fn id(&self) -> K {
		todo!()
	}

	fn format(&self) -> Format {
		todo!()
	}

	fn contents(&self) -> Option<&str> {
		todo!()
	}
}
