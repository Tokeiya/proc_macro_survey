use crate::contents_format::ContentsFormat;
use crate::prelude::Writer;
use std::hash::Hash;
use std::io::Write;

pub trait Node<K: PartialEq + Hash>: Writer {
	fn id(&self) -> &K;
	fn contents_format(&self) -> ContentsFormat;
	fn contents(&self) -> &str;
}
