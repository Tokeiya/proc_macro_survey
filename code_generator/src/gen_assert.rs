use crate::gen_impl::gen_impl;
use crate::to_snake::convert;
use quote::quote;
use syn::{Fields, ItemEnum, Result as SynResult, Variant, parse_quote};

enum Sample {
	Unit,
	Unnamed(i32),
	Named { value: String },
}

fn gen_func_name(variant: &Variant) -> syn::Ident {
	let ident = variant.ident.to_string();
	let mut buff = "assert_".to_string();
	let conv = convert(&ident);

	buff.push_str(&conv);

	syn::Ident::new(&buff, variant.ident.span())
}

fn gen_function_body(variant: &Variant, enum_ident: &syn::Ident) -> proc_macro2::TokenStream {
	let ident = &variant.ident;

	let pattern = match &variant.fields {
		Fields::Unit => quote! { #enum_ident::#ident },
		Fields::Unnamed(_) => quote! { #enum_ident::#ident(..) },
		Fields::Named(_) => quote! { #enum_ident::#ident { .. } },
	};
	quote! {
		match self {
			#pattern => {},
			_ => unreachable!(),
		}
	}
}

fn gen_function(variant: &Variant, enum_ident: &syn::Ident) -> proc_macro2::TokenStream {
	let name = gen_func_name(variant);
	let body = gen_function_body(variant, enum_ident);

	quote! {
		pub fn #name(&self){
			#body
		}
	}
}

pub fn gen_assert(scr: &str) -> SynResult<String> {
	let ast = syn::parse_str::<ItemEnum>(scr)?;

	let enum_ident = &ast.ident;
	let variants = &ast.variants;

	let body = variants
		.iter()
		.map(|variant| gen_function(variant, enum_ident))
		.collect();

	let attr: [syn::Attribute; _] = [parse_quote!(#[cfg(test)])];

	let output = gen_impl(&body, &enum_ident, attr.as_slice(), &ast.generics);

	Ok(output?.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn gen_test() {
		let actual = gen_assert(
			r##"
	enum Sample {
	Unit,
	Unnamed(i32),
	Named { value: String }
	}
	"##,
		)
		.unwrap();

		const EXPECTED: &str = r##"# [cfg (test)] impl Sample { pub fn assert_unit (& self) { match self { Sample :: Unit => { } , _ => unreachable ! () , } } pub fn assert_unnamed (& self) { match self { Sample :: Unnamed (..) => { } , _ => unreachable ! () , } } pub fn assert_named (& self) { match self { Sample :: Named { .. } => { } , _ => unreachable ! () , } } }"##;

		println!("{}", &actual);

		assert_eq!(&actual, EXPECTED);
	}
}
