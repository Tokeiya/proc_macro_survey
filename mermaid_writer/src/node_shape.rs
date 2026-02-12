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

impl Render for Shape {
	fn render(&self, writer: &mut dyn Write) -> crate::error::Result<()> {
		let value = match self {
			Shape::Rect => "rect",
			Shape::Rounded => "rounded",
			Shape::Stadium => "stadium",
			Shape::SubProc => "subroutine",
			Shape::Cylinder => "cylinder",
			Shape::Circle => "circle",
			Shape::Odd => "odd",
			Shape::Diamond => "diamond",
			Shape::Hex => "hexagon",
			Shape::LeanR => "lean-r",
			Shape::LeanL => "lean-l",
			Shape::TrabB => "trap-b",
			Shape::TrapT => "trap-t",
			Shape::DoubleCircle => "dbl-circ",
			Shape::Other(o) => o,
		};

		write!(writer, "{}", value)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::render::test_helper::assert_render;
	#[test]
	fn render() {
		assert_render(&Shape::Rect, "rect");
		assert_render(&Shape::Rounded, "rounded");
		assert_render(&Shape::Stadium, "stadium");
		assert_render(&Shape::SubProc, "subroutine");
		assert_render(&Shape::Cylinder, "cylinder");
		assert_render(&Shape::Circle, "circle");
		assert_render(&Shape::Odd, "odd");
		assert_render(&Shape::Diamond, "diamond");
		assert_render(&Shape::Hex, "hexagon");
		assert_render(&Shape::LeanR, "lean-r");
		assert_render(&Shape::LeanL, "lean-l");
		assert_render(&Shape::TrabB, "trap-b");
		assert_render(&Shape::TrapT, "trap-t");
		assert_render(&Shape::DoubleCircle, "dbl-circ");
		assert_render(&Shape::Other("custom-shape".to_string()), "custom-shape");
	}
}
