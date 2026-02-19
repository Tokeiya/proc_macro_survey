extern crate core;
use mermaid_writer::error::{Error as MermaidError, Result as MermaidResult};
use mermaid_writer::node_shape::Shape;
use mermaid_writer::prelude::*;
use mermaid_writer::regular_link::RegularLink;
use mermaid_writer::regular_node::RegularNode;
use playground::prelude::{ElementLink, ElementNode, IdGen, Integer};
use quote::{ToTokens, quote};
use syn::{
	Attribute, Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, Type, TypeArray, Variant,
	parse_quote,
};

#[cfg(feature = "dummy_a")]
#[cfg(feature = "dummy_a")]
pub enum EnumSample<'a, 'b, T: 'b>
where
	'a: 'b,
{
	#[cfg(feature = "dummy_b")]
	#[doc = "document"]
	TupleVariant(
		#[cfg(feature = "dummy_b")] (&'b T, i32),
		#[cfg(feature = "dummy_b")] &'a str,
	),
	#[cfg(feature = "dummy_c")]
	NamedVariant {
		#[cfg(feature = "dummy_c")]
		reference: &'a Integer,
		value: i32,
		#[doc = "foo"]
		tuple: (f32, f64),
	},
	#[cfg(feature = "dummy_d")]
	UnitVariant,
}

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
		TupleVariant(
			#[cfg(feature = "dummy_b")] (&'b T, i32),
			#[cfg(feature = "dummy_b")] &'a str,
		),
		#[cfg(feature = "dummy_c")]
		NamedVariant {
			#[cfg(feature = "dummy_c")]
			reference: &'a Integer,
			value: i32,
			#[doc = "foo"]
			tuple: (f32, f64),
		},
		#[cfg(feature = "dummy_d")]
		UnitVariant,
	}
		};

	let mut file = std::fs::File::create("output.mmd").unwrap();

	let mut flowchart = Flowchart::<Integer>::new("enum".to_string(), Orientation::LeftToRight);
	let mut id_gen = IdGen::default();

	_ = enum_proc(&input, &mut flowchart, &mut id_gen);

	flowchart.ord_write(&mut file).unwrap();
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
	let node = ElementNode::from_token(id_gen.next(), &data.ident, NodeShape::Hex);

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
	let node = ElementNode::from_token(id_gen.next(), &attr, Shape::SubProc);
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
	let node = ElementNode::from_token(id_gen.next(), &variant.ident, Shape::Stadium);
	let cursor = flow.add_node(node)?;
	let link = ElementLink::new(*parent, cursor, "variant");
	flow.add_link(link)?;

	for attr in variant.attrs.iter() {
		attr_proc(attr, &cursor, flow, id_gen)?
	}

	match &variant.fields {
		Fields::Named(n) => named_proc(n, &cursor, flow, id_gen)?,
		Fields::Unnamed(u) => unnamed_proc(u, &cursor, flow, id_gen)?,
		Fields::Unit => {}
	}

	Ok(())
}

fn named_proc(
	fields: &FieldsNamed,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	for elem in fields.named.iter() {
		let cursor = id_gen.next();
		let node = ElementNode::from_token(cursor, &elem.ident, Shape::Odd);
		let link = ElementLink::new(*parent, cursor, "field");

		flow.add_node(node)?;
		flow.add_link(link)?;

		for attr in elem.attrs.iter() {
			attr_proc(attr, &cursor, flow, id_gen)?
		}
	}

	Ok(())
}

fn type_array_proc(
	ty: &TypeArray,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_str(cursor.clone(), "arr", Shape::Cylinder);
	let link = ElementLink::new(*parent, cursor, "type");
	flow.add_node(node)?;
	flow.add_link(link)?;

	type_proc(&ty.elem, &cursor, flow, id_gen)
}

fn type_proc(
	ty: &Type,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();

	match ty {
		Type::Array(x) => type_array_proc(x, &parent, flow, id_gen),
		Type::BareFn(x) => todo!(),
		Type::Group(x) => todo!(),
		Type::ImplTrait(x) => todo!(),
		Type::Infer(x) => todo!(),
		Type::Macro(x) => todo!(),
		Type::Never(x) => todo!(),
		Type::Paren(x) => todo!(),
		Type::Path(x) => todo!(),
		Type::Ptr(x) => todo!(),
		Type::Reference(x) => todo!(),
		Type::Slice(x) => todo!(),
		Type::TraitObject(x) => todo!(),
		Type::Tuple(x) => todo!(),
		Type::Verbatim(x) => todo!(),
		_ => todo!(),
	}
}

fn unnamed_proc(
	fields: &FieldsUnnamed,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	for elem in fields.unnamed.iter() {
		let cursor = id_gen.next();

		let node = ElementNode::from_token(cursor, &elem.ty, Shape::Odd);

		let link = ElementLink::new(*parent, cursor, "field");
		flow.add_node(node)?;
		flow.add_link(link)?;

		for attr in elem.attrs.iter() {
			attr_proc(attr, &cursor, flow, id_gen)?
		}
	}

	Ok(())
}
