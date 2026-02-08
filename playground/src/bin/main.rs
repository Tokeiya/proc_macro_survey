use quote::ToTokens;
use proc_macro::*;
use syn::{Data, DeriveInput, Fields, Type, parse_quote, DataEnum, Variant, FieldsUnnamed};



enum Foo{
	
	Tuple(#[doc="foo"]#[cfg(dummy_a)] (i32,i32),String),
	Anonymous{value:i32,name:String}
}



fn main() {
	let input:DeriveInput = parse_quote! {
enum Foo{
	
	Tuple(#[doc="foo"]#[cfg(dummy_a)] (i32,i32),String),
	Anonymous{value:i32,name:String}
}
    };
	
	println!("{}",input.to_token_stream());
	
	if let Data::Enum(data)=input.data{
		for variant in data.variants.iter(){
			match &(variant.fields) {
				Fields::Named(x) => {}
				Fields::Unnamed(x) => {}
				Fields::Unit => {}
			}
		}
	}
	
}

fn proc_unnnamed(data:&FieldsUnnamed){
	for fld in data.unnamed.iter(){
	
	}
}

#[cfg(test)]
mod tests {
	use super::*;

}
