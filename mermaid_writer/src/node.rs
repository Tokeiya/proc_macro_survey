use crate::contents_format::ContentsFormat;
use crate::prelude::Render;
use std::hash::Hash;
use std::io::Write;

pub trait Node<K: PartialEq + Hash + Clone>: Render {
	fn id(&self) -> &K;
	fn format(&self) -> ContentsFormat;
	fn contents(&self) -> Option<&str>;
}
