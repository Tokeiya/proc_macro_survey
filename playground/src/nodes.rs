use crate::id_gen::Integer;
use mermaid_writer::flowcharts;
use mermaid_writer::line_style::Style;
use mermaid_writer::link::Link;
use mermaid_writer::node::Node as OtherNode;
use mermaid_writer::node_shape::Shape;
use mermaid_writer::prelude::{
	ArrowShape, ContentsFormat, Direction, LineShape, Node, NodeShape, Render,
};
use mermaid_writer::regular_link::RegularLink;
use mermaid_writer::regular_node::RegularNode;
use proc_macro2::Ident;
use quote::{ToTokens, quote};
use std::io::Write;

pub struct ElementNode(RegularNode<Integer>);

impl ElementNode {
	pub fn new(id: Integer, contents: &impl ToTokens, shape: NodeShape) -> Self {
		Self(RegularNode::new(id, shape, Some(to_string(&contents))))
	}
}

fn to_string(token: &impl ToTokens) -> String {
	let quoted = quote! {#token};
	quoted
		.to_string()
		.replace("\"", "\\\"")
		.replace("#", "#35;")
}

impl Render for ElementNode {
	fn render(&self, write: &mut dyn Write) -> mermaid_writer::error::Result<(), ()> {
		self.0.render(write)
	}
}

impl Node<Integer> for ElementNode {
	fn id(&self) -> Integer {
		self.0.id()
	}

	fn shape(&self) -> Shape {
		self.0.shape()
	}

	fn format(&self) -> mermaid_writer::prelude::ContentsFormat {
		self.0.format()
	}

	fn contents(&self) -> Option<&str> {
		self.0.contents()
	}
}

pub struct ElementLink(RegularLink<Integer>);

impl ElementLink {
	pub fn new(scr: Integer, tgt: Integer, description: &str) -> Self {
		Self(RegularLink::oneway(
			scr,
			tgt,
			LineShape::Normal,
			ArrowShape::Arrow,
			Some((ContentsFormat::Text, description.to_string())),
		))
	}
}

impl Render for ElementLink {
	fn render(&self, write: &mut dyn Write) -> mermaid_writer::error::Result<(), ()> {
		self.0.render(write)
	}
}

impl Link<Integer> for ElementLink {
	fn source(&self) -> Integer {
		self.0.source()
	}

	fn target(&self) -> Integer {
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

	fn format(&self) -> Option<ContentsFormat> {
		self.0.format()
	}

	fn contents(&self) -> Option<&str> {
		self.0.contents()
	}
}
