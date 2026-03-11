use std::borrow::Cow;
use syn::parse_quote;

pub struct Args<'a> {
	pub ident: syn::Ident,
	pub attrs: Cow<'a, [syn::Attribute]>,
	pub generics: syn::Generics,
}

impl<'a> Args<'a> {
	pub fn new(
		ident: syn::Ident,
		attrs: Cow<'a, [syn::Attribute]>,
		generics: syn::Generics,
	) -> Self {
		Self {
			ident,
			attrs,
			generics,
		}
	}

	pub fn from_single_attr(ident: syn::Ident, attrs: &str, generics: syn::Generics) -> Self {
		let attrs: Cow<[syn::Attribute]> = Cow::from(vec![parse_quote!(#[cfg(test)])]);

		todo!()
	}
}
