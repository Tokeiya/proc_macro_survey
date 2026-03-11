use syn::{Fields, ItemEnum, Result as SynResult, Variant};

fn gen_function_body(variant: &Variant, enum_ident: &syn::Ident) -> proc_macro2::TokenStream {
	let ident = &variant.ident;

	match &variant.fields {
		Fields::Named(named) => {}
		Fields::Unnamed(unnamed) => {}
		Fields::Unit => {}
	}

	todo!()
}

pub fn gen_assume(src: &str, attributes: &[&str]) -> SynResult<String> {
	let ast = syn::parse_str::<ItemEnum>(src)?;
	let ident = ast.ident;
	let variant = ast.variants;

	todo!()
}
