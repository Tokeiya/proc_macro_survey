use crate::contents_format::Format;
use crate::key::Key;
use crate::prelude::Render;

pub trait Node<K: Key>: Render {
	fn id(&self) -> K;
	fn format(&self) -> Format;
	fn contents(&self) -> Option<&str>;
}
