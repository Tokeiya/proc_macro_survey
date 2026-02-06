mod add_check;
mod character_manipulator;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Item, ItemEnum};


#[proc_macro_attribute]
pub fn add_check(_: TokenStream, input: TokenStream) -> TokenStream {
	add_check::add_check(input)
}