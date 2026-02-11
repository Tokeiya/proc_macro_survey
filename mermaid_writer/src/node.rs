use crate::contents_format::Format;
use crate::prelude::Render;
use std::hash::Hash;
use std::io::Write;

pub trait Node<K: PartialEq + Hash>: Render {
	fn id(&self) -> K;
	fn format(&self) -> Format;
	fn contents(&self) -> Option<&str>;
}
