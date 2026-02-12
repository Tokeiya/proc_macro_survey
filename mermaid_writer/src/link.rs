use crate::arrow_shape::Shape as ArrowShape;
use crate::contents_format::Format;
use crate::direction::Direction;
use crate::key::Key;
use crate::line_shape::Shape as LineShape;
use crate::render::Render;

pub trait Link<K: Key>: Render {
	fn source(&self) -> K;
	fn target(&self) -> K;
	fn direction(&self) -> Direction;
	fn line_shape(&self) -> LineShape;
	fn arrow_shape(&self) -> Option<ArrowShape>;
	fn format(&self) -> Format;
	fn contents(&self) -> Option<&str>;
}
