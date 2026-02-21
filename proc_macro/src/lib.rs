mod add_assume;
mod add_check;
mod character_manipulator;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn dummy_a(_: TokenStream, input: TokenStream) -> TokenStream {
	input
}

#[proc_macro_attribute]
pub fn dummy_b(_: TokenStream, input: TokenStream) -> TokenStream {
	input
}

#[proc_macro_attribute]
pub fn dummy_c(_: TokenStream, input: TokenStream) -> TokenStream {
	input
}

#[proc_macro_attribute]
pub fn dummy_d(_: TokenStream, input: TokenStream) -> TokenStream {
	input
}

#[proc_macro_attribute]
pub fn add_check(_: TokenStream, input: TokenStream) -> TokenStream {
	add_check::add_check(input)
}

#[proc_macro_attribute]
pub fn add_assume(_: TokenStream, input: TokenStream) -> TokenStream {
	add_assume::add_assume(input)
}
