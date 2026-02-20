extern crate core;
use mermaid_writer::error::{Error as MermaidError, Result as MermaidResult};
use mermaid_writer::node_shape::Shape;
use mermaid_writer::prelude::*;
use mermaid_writer::regular_link::RegularLink;
use mermaid_writer::regular_node::RegularNode;
use playground::prelude::{ElementLink, ElementNode, IdGen, Integer};
use quote::{ToTokens, quote};
use syn::{
	Attribute, Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, Lifetime, ReturnType, Type,
	TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeInfer, TypeMacro, TypeNever, TypeParen,
	TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject, TypeTuple, Variant, parse_quote,
};

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
				tuple: (std::f32, f64),
			},
			#[cfg(feature = "dummy_d")]
			UnitVariant,
			BareFnA(fn(i32, i32) -> i32),
			BareFnB(fn()->()),
			BareFnC(fn()),
			BareFnD(extern "C" fn(i32,i32)->i32),
			Array([i32; 10]),
			Slice(&'a [T]),
		}
	};

	let mut file = std::fs::File::create("output.mmd").unwrap();

	let mut flowchart = Flowchart::<Integer>::new("enum".to_string(), Orientation::LeftToRight);
	let mut id_gen = IdGen::default();

	_ = enum_proc(&input, &mut flowchart, &mut id_gen).unwrap();

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
	let link = ElementLink::new(*parent, cursor, "var");
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
		let link = ElementLink::new(*parent, cursor, "fld");

		flow.add_node(node)?;
		flow.add_link(link)?;

		for attr in elem.attrs.iter() {
			attr_proc(attr, &cursor, flow, id_gen)?
		}

		type_proc(&elem.ty, &cursor, flow, id_gen)?
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
	let node = ElementNode::from_token(cursor.clone(), &ty, Shape::Cylinder);
	let link = ElementLink::new(*parent, cursor, "arr");
	flow.add_node(node)?;
	flow.add_link(link)?;

	{
		let cur = id_gen.next();
		let node = ElementNode::from_token(cur.clone(), &ty.len, Shape::Rounded);
		let link = ElementLink::new(cursor.clone(), cur, "len");
		flow.add_node(node)?;
		flow.add_link(link)?;
	}

	type_proc(&ty.elem, &cursor, flow, id_gen)
}

fn bare_fn_proc(
	ty: &TypeBareFn,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Cylinder);
	let link = ElementLink::new(*parent, cursor, "bfn");
	flow.add_node(node)?;
	flow.add_link(link)?;

	{
		for elem in ty.inputs.iter() {
			let cur = id_gen.next();
			let node = ElementNode::from_token(cur.clone(), &elem, Shape::Rounded);
			let link = ElementLink::new(cursor.clone(), cur, "arg");
			flow.add_node(node)?;
			flow.add_link(link)?;

			type_proc(&elem.ty, &cur, flow, id_gen)?;
		}

		let cur = id_gen.next();

		let node = match &ty.output {
			ReturnType::Default => ElementNode::from_str(cur.clone(), "Default", Shape::Card),
			ReturnType::Type(_, t) => ElementNode::from_token(cur.clone(), t, Shape::Rounded),
		};

		let link = ElementLink::new(cursor.clone(), cur, "ret");
		flow.add_node(node)?;
		flow.add_link(link)?;

		if let Some(abi) = &ty.abi {
			let cur = id_gen.next();
			let node = ElementNode::from_token(cur.clone(), abi, Shape::Card);
			let link = ElementLink::new(cursor.clone(), cur, "abi");
			flow.add_node(node)?;
			flow.add_link(link)?;
		}
	}

	Ok(())
}

fn group_proc(
	ty: &TypeGroup,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "grp");
	flow.add_node(node)?;
	flow.add_link(link)?;

	type_proc(&ty.elem, &cursor, flow, id_gen)
}

fn impl_trait_proc(
	ty: &TypeImplTrait,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "impl");
	flow.add_node(node)?;
	flow.add_link(link)?;

	Ok(())
}

fn infer_proc(
	ty: &TypeInfer,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "infer");
	flow.add_node(node)?;
	flow.add_link(link)?;

	Ok(())
}

fn macro_proc(
	ty: &TypeMacro,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "macro");
	flow.add_node(node)?;
	flow.add_link(link)?;

	Ok(())
}

fn never_proc(
	ty: &TypeNever,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "never");
	flow.add_node(node)?;
	flow.add_link(link)?;

	Ok(())
}

fn paren_proc(
	ty: &TypeParen,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "paren");
	flow.add_node(node)?;
	flow.add_link(link)?;

	type_proc(&ty.elem, &cursor, flow, id_gen)
}

fn path_proc(
	ty: &TypePath,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "path");
	flow.add_node(node)?;
	flow.add_link(link)?;

	Ok(())
}

fn ptr_proc(
	ty: &TypePtr,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "Ptr");
	flow.add_node(node)?;
	flow.add_link(link)?;

	type_proc(&ty.elem, &cursor, flow, id_gen)
}

fn lifetime_proc(
	lifetime: &Option<Lifetime>,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	if let Some(lt) = lifetime {
		let cursor = id_gen.next();

		let node = ElementNode::from_token(cursor.clone(), lt, Shape::Rounded);
		let link = ElementLink::new(*parent, cursor, "lft");

		flow.add_node(node)?;
		flow.add_link(link)?;
	}

	Ok(())
}

fn reference_proc(
	ty: &TypeReference,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "ref");
	flow.add_node(node)?;
	flow.add_link(link)?;

	lifetime_proc(&ty.lifetime, &cursor, flow, id_gen)?;
	type_proc(&ty.elem, &cursor, flow, id_gen)
}

fn slice_proc(
	ty: &TypeSlice,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "slice");
	flow.add_node(node)?;
	flow.add_link(link)?;

	type_proc(&ty.elem, &cursor, flow, id_gen)
}

fn trait_object_proc(
	ty: &TypeTraitObject,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "tobj");
	flow.add_node(node)?;
	flow.add_link(link)?;

	Ok(())
}

fn tuple_proc(
	ty: &TypeTuple,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	let cursor = id_gen.next();
	let node = ElementNode::from_token(cursor.clone(), ty, Shape::Rounded);
	let link = ElementLink::new(*parent, cursor, "tuple");
	flow.add_node(node)?;
	flow.add_link(link)?;

	let tmp = &ty.elems.iter().len();

	dbg!(tmp);

	for elem in ty.elems.iter() {
		type_proc(elem, &cursor, flow, id_gen)?
	}

	Ok(())
}

fn type_proc(
	ty: &Type,
	parent: &Integer,
	flow: &mut Flowchart<Integer>,
	id_gen: &mut IdGen,
) -> MermaidResult<(), Integer> {
	match ty {
		Type::Array(x) => type_array_proc(x, &parent, flow, id_gen),
		Type::BareFn(x) => bare_fn_proc(x, &parent, flow, id_gen),
		Type::Group(x) => group_proc(x, &parent, flow, id_gen),
		Type::ImplTrait(x) => impl_trait_proc(x, &parent, flow, id_gen),
		Type::Infer(x) => infer_proc(x, &parent, flow, id_gen),
		Type::Macro(x) => macro_proc(x, &parent, flow, id_gen),
		Type::Never(x) => never_proc(x, &parent, flow, id_gen),
		Type::Paren(x) => paren_proc(x, &parent, flow, id_gen),
		Type::Path(x) => path_proc(x, &parent, flow, id_gen),
		Type::Ptr(x) => ptr_proc(x, &parent, flow, id_gen),
		Type::Reference(x) => reference_proc(x, &parent, flow, id_gen),
		Type::Slice(x) => slice_proc(x, &parent, flow, id_gen),
		Type::TraitObject(x) => trait_object_proc(x, &parent, flow, id_gen),
		Type::Tuple(x) => tuple_proc(x, &parent, flow, id_gen),
		_ => unreachable!(),
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

		let node = ElementNode::from_token(cursor, &elem, Shape::Odd);
		let link = ElementLink::new(*parent, cursor, "fld");
		flow.add_node(node)?;
		flow.add_link(link)?;

		for attr in elem.attrs.iter() {
			attr_proc(attr, &cursor, flow, id_gen)?
		}
		type_proc(&elem.ty, &cursor, flow, id_gen)?
	}

	Ok(())
}
