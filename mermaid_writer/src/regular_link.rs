use crate::arrow_shape::{Shape as ArrowShape, Shape};
use crate::contents_format::Format;
use crate::direction::Direction;
use crate::line_shape::{LineStyle, Shape as LineShape};
use crate::prelude::*;
use crate::render::Render;
use std::hash::Hash;
use std::io::Write;

pub struct RegularLink<K> {
	source: K,
	target: K,
	direction: Direction,
	line_style: LineStyle,
	arrow_shape: Option<ArrowShape>,
	contents: Option<(Format, String)>,
}

impl<K: Key> RegularLink<K> {
	pub fn invisible(source: K, target: K) -> Self {
		Self {
			source,
			target,
			direction: Direction::Open,
			line_style: LineStyle::Invisible,
			arrow_shape: None,
			contents: None,
		}
	}

	pub fn oneway(
		source: K,
		target: K,
		line_shape: LineShape,
		arrow_shape: ArrowShape,
		contents: Option<(Format, String)>,
	) -> Self {
		Self {
			source,
			target,
			direction: Direction::Oneway,
			line_style: LineStyle::Visible(line_shape),
			arrow_shape: Some(arrow_shape),
			contents,
		}
	}

	pub fn both(
		source: K,
		target: K,
		line_shape: LineShape,
		arrow_shape: ArrowShape,
		contents: Option<(Format, String)>,
	) -> Self {
		Self {
			source,
			target,
			direction: Direction::Both,
			line_style: LineStyle::Visible(line_shape),
			arrow_shape: Some(arrow_shape),
			contents,
		}
	}

	pub fn open(
		source: K,
		target: K,
		line_shape: LineShape,
		contents: Option<(Format, String)>,
	) -> Self {
		Self {
			source,
			target,
			direction: Direction::Open,
			line_style: LineStyle::Visible(line_shape),
			arrow_shape: None,
			contents,
		}
	}

	pub fn source(&self) -> K {
		self.source.clone()
	}

	pub fn target(&self) -> K {
		self.target.clone()
	}

	pub fn line_style(&self) -> LineStyle {
		self.line_style
	}

	pub fn arrow_shape(&self) -> Option<ArrowShape> {
		self.arrow_shape
	}

	pub fn direction(&self) -> Direction {
		self.direction
	}

	pub fn format(&self) -> Option<Format> {
		match &self.contents {
			None => None,
			Some((f, _)) => Some(*f),
		}
	}

	pub fn contents(&self) -> Option<&str> {
		match &self.contents {
			None => None,
			Some((_, c)) => Some(c.as_str()),
		}
	}

	fn textless_render(&self, write: &mut dyn Write) -> Result<()> {
		if self.arrow_shape.is_none() {
			match self.line_style {
				LineStyle::Visible(LineShape::Normal) => write!(write, "---")?,
				LineStyle::Visible(LineShape::Thick) => write!(write, "===")?,
				LineStyle::Visible(LineShape::Dotted) => write!(write, "-.-")?,
				LineStyle::Invisible => write!(write, "~~~")?,
			}
		} else {
			match self.line_style {
				LineStyle::Visible(LineShape::Normal) => write!(write, "--")?,
				LineStyle::Visible(LineShape::Thick) => write!(write, "==")?,
				LineStyle::Visible(LineShape::Dotted) => write!(write, "-.-")?,
				LineStyle::Invisible => unreachable!(),
			}
		}

		Ok(())
	}

	fn text_render(&self, write: &mut dyn Write) -> Result<()> {
		let shape = match self.line_style {
			LineStyle::Visible(s) => s,
			LineStyle::Invisible => unreachable!(),
		};

		match shape {
			LineShape::Normal => write!(write, "--")?,
			LineShape::Thick => write!(write, "==")?,
			LineShape::Dotted => write!(write, "-.")?,
		}

		match self.format().unwrap() {
			Format::Text => write!(write, " \"")?,
			Format::Markdown => write!(write, " \"`")?,
		}

		write!(write, "{}", self.contents().unwrap())?;

		match self.format().unwrap() {
			Format::Text => write!(write, "\" ")?,
			Format::Markdown => write!(write, "`\" ")?,
		}

		match shape {
			LineShape::Normal => write!(write, "--")?,
			LineShape::Thick => write!(write, "==")?,
			LineShape::Dotted => write!(write, ".-")?,
		}
		Ok(())
	}
}

impl<K: Key> Render for RegularLink<K> {
	fn render(&self, write: &mut dyn Write) -> Result<()> {
		self.source.render(write)?;

		match self.arrow_shape {
			None => write!(write, " ")?,
			Some(ArrowShape::Arrow) => write!(write, " <")?,
			Some(ArrowShape::Cross) => write!(write, " x")?,
			Some(ArrowShape::Circle) => write!(write, " o")?,
		}

		if self.contents.is_some() {
			self.text_render(write)?;
		} else {
			self.textless_render(write)?;
		};

		match self.arrow_shape {
			None => write!(write, " ")?,
			Some(ArrowShape::Arrow) => write!(write, "> ")?,
			Some(ArrowShape::Cross) => write!(write, "x ")?,
			Some(ArrowShape::Circle) => write!(write, "o ")?,
		}

		self.target.render(write)?;
		write!(write, "\n")?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::key::test_helper;
	use crate::render::test_helper::assert_render;

	#[test]
	fn invisible() {
		let fixture = RegularLink::invisible(10, 20);
		assert_eq!(fixture.direction, Direction::Open);
		assert_eq!(fixture.line_style, LineStyle::Invisible);
		assert_eq!(fixture.arrow_shape, None);
		assert_eq!(fixture.contents, None);
	}

	#[test]
	fn oneway() {
		let fixture = RegularLink::oneway(
			"node1".to_string(),
			"node2".to_string(),
			LineShape::Thick,
			ArrowShape::Cross,
			Some((Format::Text, "link".to_string())),
		);

		assert_eq!(fixture.source, "node1".to_string());
		assert_eq!(fixture.target, "node2".to_string());
		assert_eq!(fixture.direction, Direction::Oneway);
		assert!(matches!(
			fixture.line_style,
			LineStyle::Visible(LineShape::Thick)
		));
		assert_eq!(fixture.arrow_shape, Some(ArrowShape::Cross));
		assert_eq!(fixture.contents, Some((Format::Text, "link".to_string())));
	}

	#[test]
	fn both() {
		let fixture = RegularLink::both(10, 20, LineShape::Dotted, ArrowShape::Arrow, None);

		assert_eq!(fixture.source, 10);
		assert_eq!(fixture.target, 20);
		assert_eq!(fixture.direction, Direction::Both);
		assert!(matches!(
			fixture.line_style,
			LineStyle::Visible(LineShape::Dotted)
		));
		assert_eq!(fixture.arrow_shape, Some(ArrowShape::Arrow));
		assert_eq!(fixture.contents, None);
	}

	#[test]
	fn open() {
		let fixture = RegularLink::open(
			"start".to_string(),
			"end".to_string(),
			LineShape::Normal,
			Some((Format::Markdown, "open link".to_string())),
		);

		assert_eq!(fixture.source, "start".to_string());
		assert_eq!(fixture.target, "end".to_string());
		assert_eq!(fixture.direction, Direction::Open);
		assert!(matches!(
			fixture.line_style,
			LineStyle::Visible(LineShape::Normal)
		));
		assert_eq!(fixture.arrow_shape, None);
		assert_eq!(
			fixture.contents,
			Some((Format::Markdown, "open link".to_string()))
		);
	}

	#[test]
	fn source() {
		let fixture = RegularLink::oneway(1, 2, LineShape::Normal, ArrowShape::Arrow, None);
		assert_eq!(fixture.source(), 1);
	}

	#[test]
	fn target() {
		let fixture = RegularLink::oneway(1, 2, LineShape::Normal, ArrowShape::Arrow, None);
		assert_eq!(fixture.target(), 2);
	}

	#[test]
	fn line_style() {
		let fixture = RegularLink::oneway(1, 2, LineShape::Normal, ArrowShape::Arrow, None);
		assert!(matches!(
			fixture.line_style(),
			LineStyle::Visible(LineShape::Normal)
		));

		let fixture = RegularLink::invisible(1, 2);
		assert_eq!(fixture.line_style(), LineStyle::Invisible);
	}

	#[test]
	fn arrow_shape() {
		let fixture = RegularLink::oneway(1, 2, LineShape::Normal, ArrowShape::Arrow, None);
		assert_eq!(fixture.arrow_shape(), Some(ArrowShape::Arrow));

		let fixture = RegularLink::open(1, 2, LineShape::Normal, None);
		assert_eq!(fixture.arrow_shape(), None);

		let fixture = RegularLink::invisible(1, 2);
		assert_eq!(fixture.arrow_shape(), None);
	}

	#[test]
	fn direction() {
		let fixture = RegularLink::oneway(1, 2, LineShape::Normal, ArrowShape::Arrow, None);
		assert_eq!(fixture.direction(), Direction::Oneway);

		let fixture = RegularLink::both(1, 2, LineShape::Normal, ArrowShape::Arrow, None);
		assert_eq!(fixture.direction(), Direction::Both);

		let fixture = RegularLink::open(1, 2, LineShape::Normal, None);
		assert_eq!(fixture.direction(), Direction::Open);

		let fixture = RegularLink::invisible(1, 2);
		assert_eq!(fixture.direction(), Direction::Open);
	}

	#[test]
	fn format() {
		let fixture = RegularLink::oneway(
			1,
			2,
			LineShape::Normal,
			ArrowShape::Arrow,
			Some((Format::Text, "link".to_string())),
		);
		assert_eq!(fixture.format(), Some(Format::Text));

		let fixture = RegularLink::open(1, 2, LineShape::Normal, None);
		assert_eq!(fixture.format(), None);

		let fixture = RegularLink::invisible(1, 2);
		assert_eq!(fixture.format(), None);

		let fixture = RegularLink::oneway(
			1,
			2,
			LineShape::Normal,
			ArrowShape::Arrow,
			Some((Format::Markdown, "link".to_string())),
		);
		assert_eq!(fixture.format(), Some(Format::Markdown));
	}

	#[test]
	fn contents() {
		let fixture = RegularLink::oneway(
			1,
			2,
			LineShape::Normal,
			ArrowShape::Arrow,
			Some((Format::Text, "link".to_string())),
		);
		assert_eq!(fixture.contents(), Some("link"));

		let fixture = RegularLink::open(1, 2, LineShape::Normal, None);
		assert_eq!(fixture.contents(), None);

		let fixture = RegularLink::invisible(1, 2);
		assert_eq!(fixture.contents(), None);

		let fixture = RegularLink::oneway(
			1,
			2,
			LineShape::Normal,
			ArrowShape::Arrow,
			Some((Format::Markdown, "link".to_string())),
		);
		assert_eq!(fixture.contents(), Some("link"));
	}

	#[test]
	fn render() {
		let fixture = RegularLink::invisible(10, 20);
		assert_render(&fixture, "10 ~~~ 20\n");

		let fixture = RegularLink::oneway(
			"node1".to_string(),
			"node2".to_string(),
			LineShape::Thick,
			ArrowShape::Cross,
			Some((Format::Text, "link".to_string())),
		);

		assert_render(&fixture, "node1 x== \"link\" ==x node2\n");

		let fixture = RegularLink::both(
			10,
			20,
			LineShape::Dotted,
			ArrowShape::Circle,
			Some((Format::Markdown, "markdown".to_string())),
		);
		assert_render(&fixture, "10 o-. \"`markdown`\" .-o 20\n");
	}
}
