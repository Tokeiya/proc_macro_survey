extern crate core;

use mermaid_writer::error::{Error as MermaidError, Result as MermaidResult};
use mermaid_writer::node_shape::Shape;
use mermaid_writer::prelude::*;
use mermaid_writer::regular_link::RegularLink;
use mermaid_writer::regular_node::RegularNode;
use playground::prelude::{ElementLink, ElementNode, IdGen, Integer};
use quote::{ToTokens, quote};
use std::fs;
use std::io::Write;
use syn::token::In;
use syn::{Attribute, ItemEnum, Variant, parse_quote};

pub fn main() {
	let input: ItemEnum = parse_quote! {
		#[cfg(feature = "dummy_a")]
	#[cfg(feature = "dummy_a")]
	pub enum EnumSample<'a, 'b, T: 'b>
	where
		'a: 'b,
	{
		#[cfg(feature = "dummy_b")]
		#[doc = "document"]
		TupleVariant((&'b T, i32), &'a str),
		#[cfg(feature = "dummy_c")]
		NamedVariant {
			reference: &'a Envelope,
			value: i32,
			tuple: (f32, f64),
		},
		#[cfg(feature = "dummy_d")]
		UnitVariant,
	}};

	let mut file = std::fs::File::create("output.md").unwrap();
	_ = writeln!(&file, r#"```mermaid"#);

	let mut flowchart = Flowchart::<Integer>::new("enum".to_string(), Orientation::BottomToTop);
	let mut id_gen = IdGen::default();

	_ = enum_proc(&input, &mut flowchart, &mut id_gen);

	flowchart.write(&mut file).unwrap();
	_ = writeln!(&file, "```");
}

fn to_string(token: &impl ToTokens) -> String {
	let quoted = quote! {#token};
	quoted.to_string()
}

fn enum_proc(
	data: &ItemEnum,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let node = ElementNode::new(id_gen.next(), &data.ident, NodeShape::Hex);

	flow.add_node(node)?;
	let parent = id_gen.current();

	let (i, t, w) = data.generics.split_for_impl();

	let node = RegularNode::new(id_gen.next(), Shape::LeanL, Some(to_string(&i)));
	let cursor = flow.add_node(node)?;

	let link = ElementLink::new(parent, cursor, "impl");
	flow.add_link(link)?;

	let node = RegularNode::new(id_gen.next(), Shape::LeanL, Some(to_string(&t)));
	let cursor = flow.add_node(node)?;
	let link = ElementLink::new(parent, cursor, "type");
	flow.add_link(link)?;

	if let Some(w) = w {
		let node = RegularNode::new(id_gen.next(), Shape::LeanL, Some(to_string(&w)));
		let cursor = flow.add_node(node)?;
		let link = ElementLink::new(parent, cursor, "where");
		flow.add_link(link)?;
	}

	for att in data.attrs.iter() {
		attr_proc(att, &parent, flow, id_gen)?
	}

	for variant in data.variants.iter() {
		variant_proc(variant, &parent, flow, id_gen)?
	}

	Ok(())
}

fn attr_proc(
	attr: &Attribute,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let node = ElementNode::new(id_gen.next(), &attr, Shape::SubProc);
	let cursor = flow.add_node(node)?;
	let link = ElementLink::new(*parent, cursor, "attr");
	flow.add_link(link)?;

	Ok(())
}

fn variant_proc(
	variant: &Variant,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let node = ElementNode::new(id_gen.next(), &variant.ident, Shape::Stadium);
	let cursor = flow.add_node(node)?;
	let link = ElementLink::new(*parent, cursor, "variant");
	flow.add_link(link)?;

	for attr in variant.attrs.iter() {
		attr_proc(attr, &cursor, flow, id_gen)?
	}

	Ok(())
}
