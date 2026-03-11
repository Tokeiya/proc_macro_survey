use proc_macro2::TokenStream;
use quote::quote;
use syn::Result as SynResult;

pub fn gen_impl(
	body: &TokenStream,
	ident: &syn::Ident,
	attrs: &[syn::Attribute],
	generics: &syn::Generics,
) -> SynResult<TokenStream> {
	let attrs = attrs.iter().map(|attr| quote! {#attr});

	let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();

	let output = quote! {
		#(#attrs)*
		impl #impl_gen #ident #ty_gen
		#where_clause
		{
			#body
		}
	};

	Ok(output)
}
