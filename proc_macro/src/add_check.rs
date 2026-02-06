use crate::character_manipulator::to_lower_snake;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemEnum, Variant, parse_macro_input};

fn gen_func_name(variant: &Variant) -> syn::Ident {
	let ident = variant.ident.to_string();
	let mut buff = "is_".to_string();
	to_lower_snake(&ident, &mut buff);

	format_ident!("{}", buff, span = variant.ident.span())
}

fn gen_function(variant: &Variant, enum_ident: &syn::Ident) -> proc_macro2::TokenStream {
	let name = gen_func_name(variant);
	let body = gen_func_body(variant, enum_ident);

	quote! {
		pub fn #name(&self)->bool{
			#body
		}
	}
}

fn gen_func_body(variant: &Variant, enum_ident: &syn::Ident) -> proc_macro2::TokenStream {
	let ident = &variant.ident;

	let pattern = match &variant.fields {
		Fields::Unit => quote! { #enum_ident::#ident },
		Fields::Unnamed(_) => quote! { #enum_ident::#ident(..) },
		Fields::Named(_) => quote! { #enum_ident::#ident { .. } },
	};
	quote! {
		match self {
			#pattern => true,
			_ => false,
		}
	}
}

pub fn add_check(input: TokenStream) -> TokenStream {
	println!("enter add_check");
	let ast = parse_macro_input!(input as ItemEnum);

	let enum_ident = &ast.ident;
	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

	let variants = &ast.variants;

	let body = variants
		.iter()
		.map(|variant| gen_function(variant, enum_ident));

	let output = quote! {#ast

		#[cfg(test)]
		impl #impl_generics #enum_ident #ty_generics
		#where_clause
		{
			#(#body)*
		}
	};
	output.into()
}
