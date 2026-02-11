use crate::contents_format::ContentsFormat;
use crate::render::Render;
use std::io::Write;

pub trait Link<K>: Render {
	fn source(&self) -> K;
	fn target(&self) -> K;
	fn format(&self) -> ContentsFormat;
	fn contents(&self) -> &str;
}
