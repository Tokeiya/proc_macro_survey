use crate::prelude::*;
use std::io::Write as IoWrite;

#[derive(Debug, Clone)]
pub enum Orientation {
	TopToBottom,
	TopDown,
	BottomToTop,
	RightToLeft,
	LeftToRight,
}
