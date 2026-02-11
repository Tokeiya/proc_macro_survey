use crate::contents_format::Format;
use crate::render::Render;
use std::io::Write;

pub trait Link<K>: Render {
	fn source(&self) -> K;
	fn target(&self) -> K;
	fn format(&self) -> Format;
	fn contents(&self) -> Option<&str>;
}
