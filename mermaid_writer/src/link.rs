use crate::contents_format::Format;
use crate::direction::Direction;
use crate::key::Key;
use crate::render::Render;

pub trait Link<K: Key>: Render {
	fn source(&self) -> K;
	fn target(&self) -> K;
	fn direction(&self) -> Direction;
	fn format(&self) -> Format;
	fn contents(&self) -> Option<&str>;
}
