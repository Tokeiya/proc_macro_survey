use crate::to_snake::convert;
use quote::quote;
use syn::{Error as SynError, Fields, Item, ItemEnum, Result as SynResult, Variant};

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
	let (impl_gen, ty_gen, where_clause) = ast.generics.split_for_impl();

	let variants = &ast.variants;

	let body = variants
		.iter()
		.map(|variant| gen_function(variant, enum_ident));

	let output = quote! {
		#[cfg(test)]
		impl #impl_gen #enum_ident #ty_gen
		#where_clause
		{
			#(#body)*
		}
	};

	Ok(output.to_string())
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

		const EXPECTED: &str = r##"
impl Sample {
	pub fn is_unit(&self) -> bool {
		match self {
			Sample::Unit => true,
			_ => false,
		}
	}

	pub fn is_named(&self) -> bool {
		match self {
			Sample::Named { .. } => true,
			_ => false,
		}
	}

	pub fn is_unnamed(&self) -> bool {
		match self {
			Sample::Unnamed(_) => true,
			_ => false,
		}
	}
}
		"##;

		assert_eq!(&actual, EXPECTED);
	}
}
