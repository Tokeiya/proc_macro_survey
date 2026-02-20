use crate::visitor::id::Id;
use mermaid_writer::direction::Direction;
use mermaid_writer::line_style::{Shape, Style};
use mermaid_writer::prelude as mermaid;
use mermaid_writer::prelude::{ArrowShape, Render};
use mermaid_writer::regular_link::RegularLink;
use std::io::Write;

pub struct Link(RegularLink<Id>);

impl Link {
	pub fn new(from: Id, to: Id) -> Self {
		Self(RegularLink::oneway(
			from,
			to,
			Shape::Normal,
			ArrowShape::Arrow,
			None,
		))
	}
}

impl Render for Link {
	fn render(&self, write: &mut dyn Write) -> mermaid_writer::error::Result<(), ()> {
		self.0.render(write)
	}
}

impl mermaid::Link<Id> for Link {
	fn source(&self) -> Id {
		self.0.source()
	}

	fn target(&self) -> Id {
		self.0.target()
	}

	fn direction(&self) -> Direction {
		self.0.direction()
	}

	fn line_style(&self) -> Style {
		self.0.line_style()
	}

	fn arrow_shape(&self) -> Option<ArrowShape> {
		self.0.arrow_shape()
	}

	fn format(&self) -> Option<mermaid::ContentsFormat> {
		self.0.format()
	}

	fn contents(&self) -> Option<&str> {
		self.0.contents()
	}
}
