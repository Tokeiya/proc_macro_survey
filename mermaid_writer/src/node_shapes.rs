use crate::prelude::Render;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Shape {
	Rect,
	Rounded,
	Stadium,
	SubProc,
	Cylinder,
	Circle,
	Odd,
	Diamond,
	Hex,
	LeanR,
	LeanL,
	TrabB,
	TrapT,
	DoubleCircle,
	Other(String),
}
