use playground::prelude::*;
use proc_macro::*;
use proc_macro2::Ident;
use quote::{ToTokens, quote};
use syn::{Data, DataEnum, DeriveInput, Field, Fields, FieldsUnnamed, Type, Variant, parse_quote};
use syn::{FieldsNamed, ItemEnum};
pub struct Envelope(pub i16);

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
}

fn main() {
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

	enum_proc(&input);
}

fn enum_proc(enum_data: &ItemEnum) {
	println!("enum_proc");
	let g = &enum_data.generics;
	let (i, t, w) = enum_data.generics.split_for_impl();

	print_token(Some("gen"), &g);
	print_token(Some("impl"), &i);
	print_token(Some("ty"), &t);

	if let Some(w) = w {
		print_token(Some("where"), &w);
	}
	println!();

	for variant in enum_data.variants.iter() {
		variant_proc(variant);
	}
}

fn variant_proc(variant: &Variant) {
	println!("ident:{}", &variant.ident);

	for attr in variant.attrs.iter() {
		print_token(Some("attr"), attr);
	}

	_ = match &variant.fields {
		Fields::Named(n) => named_proc(n),
		Fields::Unnamed(u) => unnamed_proc(u),
		Fields::Unit => println!("Unit"),
	};

	println!()
}

fn named_proc(value: &FieldsNamed) {
	println!("named");
	for elem in value.named.iter() {
		print_token(Some("field"), elem);
		field_proc(elem);
	}
	println!()
}

fn unnamed_proc(value: &FieldsUnnamed) {
	println!("unnnamed");
	for elem in value.unnamed.iter() {
		print_token(Some("field"), elem);
		field_proc(elem);
	}

	println!()
}

fn field_proc(value: &Field) {
	if let Some(id)=&value.ident{
		print_token(Some("field_id"), &id)
	}else {
		println!("field_id:None");
	}
	
	
	
}

#[cfg(test)]
mod tests {
	use super::*;
}
