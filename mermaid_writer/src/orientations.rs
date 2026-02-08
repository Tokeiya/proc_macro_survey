use crate::prelude::*;
use std::fmt::Display;
use std::io::Write as IoWrite;

pub enum Orientation {
	TopToBottom,
	TopDown,
	BottomToTop,
	RightToLeft,
	LeftToRight,
}

impl Display for Orientation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", 32)
	}
}

impl<W: IoWrite> Writer<W> for Orientation {
	fn write(&self, writer: &mut W) -> Result<()> {
		let id = match self {
			Orientation::TopToBottom => "TB",
			Orientation::TopDown => "TD",
			Orientation::BottomToTop => "BT",
			Orientation::RightToLeft => "RL",
			Orientation::LeftToRight => "LR",
		};

		writer.write_fmt(format_args!("flowchart {id}\n"))?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::prelude::Orientation;

	fn assert(actual: &[u8], expected: &str) {
		assert_eq!(actual, expected.as_bytes())
	}

	#[test]
	fn write() {
		let mut buff = Vec::<u8>::new();
		let fixture = Orientation::BottomToTop;
		fixture.write(&mut buff).unwrap();
		assert(&buff, "flowchart BT\n");

		buff.clear();
		Orientation::TopToBottom.write(&mut buff).unwrap();
		assert(&buff, "flowchart TB\n");

		buff.clear();
		Orientation::RightToLeft.write(&mut buff).unwrap();
		assert(&buff, "flowchart RL\n");

		buff.clear();
		Orientation::LeftToRight.write(&mut buff).unwrap();
		assert(&buff, "flowchart LR\n");
	}
}
