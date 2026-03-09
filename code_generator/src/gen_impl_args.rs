use proc_macro2::TokenStream;
use std::borrow::Cow;
use std::sync::LazyLock;
use syn::parse_quote;

pub struct Args<'a> {
	pub body: &'a TokenStream,
	pub ident: &'a syn::Ident,
	pub attrs: Option<Cow<'a, [syn::Attribute]>>,
	pub generics: &'a syn::Generics,
}

static TEST_ATTR: LazyLock<[syn::Attribute; 1]> = LazyLock::new(|| {
	let elem = parse_quote!(#[cfg(test)]);
	[elem]
});
impl<'a> Args<'a> {
	pub fn new(
		body: &'a TokenStream,
		ident: &'a syn::Ident,
		attrs: Option<&'a [syn::Attribute]>,
		generics: &'a syn::Generics,
	) -> Self {
		let attr = if let Some(attr) = attrs {
			Some(Cow::Borrowed(attr))
		} else {
			None
		};

		Self {
			body,
			ident,
			attrs: attr,
			generics,
		}
	}

	pub fn with_test_attr(
		body: &'a TokenStream,
		ident: &'a syn::Ident,
		generics: &'a syn::Generics,
	) -> Self {
		Self::new(body, ident, Some(&[]), generics)
	}
}
