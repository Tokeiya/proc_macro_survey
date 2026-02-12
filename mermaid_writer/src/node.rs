use crate::contents_format::Format;
use crate::prelude::Render;
use std::hash::Hash;
use crate::key::Key;

pub trait Node<K: Key>: Render {
	fn id(&self) -> K;
	fn format(&self) -> Format;
	fn contents(&self) -> Option<&str>;
}
