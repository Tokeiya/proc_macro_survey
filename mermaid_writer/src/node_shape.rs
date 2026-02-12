use crate::prelude::Render;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Shape {
	pub fn shape_name(&self) -> &str {
		match self {
			Shape::Rect => "rect",
			Shape::Rounded => "rounded",
			Shape::Stadium => "stadium",
			Shape::SubProc => "subroutine",
			Shape::Cylinder => "cylinder",
			Shape::Circle => "circle",
			Shape::Odd => "odd",
			Shape::Diamond => "diamond",
			Shape::Hex => "hexagon",
			Shape::LeanR => "leanRight",
			Shape::LeanL => "leanLeft",
			Shape::TrabB => "trapezoid",
			Shape::TrapT => "trapezoid",
			Shape::DoubleCircle => "doubleCircle",
			Shape::Other(o) => o,
		}
	}
}
