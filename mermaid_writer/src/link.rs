use crate::arrow_shape::Shape as ArrowShape;
use crate::contents_format::Format;
use crate::direction::Direction;
use crate::key::Key;
use crate::line_shape::LineStyle;
use crate::render::Render;

pub trait Link<K: Key>: Render {
	fn source(&self) -> K;
	fn target(&self) -> K;
	fn direction(&self) -> Direction;
	fn line_style(&self) -> LineStyle;
	fn arrow_shape(&self) -> Option<ArrowShape>;
	fn format(&self) -> Option<Format>;
	fn contents(&self) -> Option<&str>;
}
