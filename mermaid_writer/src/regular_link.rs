use crate::contents_format::Format;
use crate::edge_shape::Shape;
use crate::link::Link;
use crate::prelude::Render;
use std::hash::Hash;
use std::io::Write;

pub struct RegularLink<K> {
	source: K,
	target: K,
	format: Format,
	contents: Option<String>,
}

impl<K: PartialEq + Hash + Clone> RegularLink<K> {
	pub fn new(
		source: K,
		target: K,
		shape: Shape,
		format: Format,
		contents: Option<String>,
	) -> Self {
		todo!()
	}

	pub fn textless_new(source: K, target: K, shape: Shape) -> Self {
		todo!()
	}
}

impl<K: PartialEq + Hash + Clone> Render for RegularLink<K> {
	fn render(&self, writer: &mut dyn Write) -> crate::error::Result<()> {
		todo!()
	}
}

impl<K: PartialEq + Hash + Clone> Link<K> for RegularLink<K> {
	fn source(&self) -> K {
		todo!()
	}

	fn target(&self) -> K {
		todo!()
	}

	fn format(&self) -> Format {
		todo!()
	}

	fn contents(&self) -> Option<&str> {
		todo!()
	}
}
