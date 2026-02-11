use crate::prelude::Writer;
use std::io::Write;

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
