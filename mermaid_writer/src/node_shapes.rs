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

impl<W: Write> Writer<W> for Shape {
	fn write(&self, writer: &mut W) -> crate::error::Result<()> {
		todo!()
	}
}

#[cfg(test)]
mod tests {}
