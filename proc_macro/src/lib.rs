mod add_assume;
mod add_check;
mod character_manipulator;

use proc_macro::TokenStream;
use syn::{Item, ItemEnum, parse_macro_input};

#[proc_macro_attribute]
pub fn add_check(_: TokenStream, input: TokenStream) -> TokenStream {
	add_check::add_check(input)
}

#[proc_macro_attribute]
pub fn add_assume(_: TokenStream, input: TokenStream) -> TokenStream {
	add_assume::add_assume(input)
}
