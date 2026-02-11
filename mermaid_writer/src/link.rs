use crate::contents_format::ContentsFormat;
use crate::writer::Writer;
use std::io::Write;

pub trait Link<K>: Writer {
	fn from(&self) -> K;
	fn source(&self) -> K;
	fn target(&self) -> ContentsFormat;
}
