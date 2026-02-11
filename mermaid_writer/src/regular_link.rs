use crate::contents_format::Format;
use crate::link::Link;
use crate::prelude::Render;
use std::hash::Hash;
use std::io::Write;

pub struct RegularLink<K> {
	key: K,
}

impl<K: PartialEq + Hash + Clone> Render for RegularLink<K> {
	fn write(&self, writer: &mut dyn Write) -> crate::error::Result<()> {
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
