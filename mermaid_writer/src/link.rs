use crate::contents_format::Format;
use crate::render::Render;

pub trait Link<K>: Render {
	fn source(&self) -> K;
	fn target(&self) -> K;
	fn format(&self) -> Format;
	fn contents(&self) -> Option<&str>;
}
