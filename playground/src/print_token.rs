use quote::{ToTokens, quote};

pub fn print_token(header: Option<&str>, token: &impl ToTokens) {
	let q = quote! {#token};

	match header {
		None => println!("{}", q),
		Some(h) => println!("{h}:{}", q),
	}
}

pub fn print_tokens(header: Option<&str>, tokens: &[&dyn ToTokens]) {
	match header {
		None => println!("---------------"),
		Some(h) => println!("------{h}-----"),
	}

	for t in tokens {
		let q = quote! {#t};
		println!("{}", q)
	}

	println!("---------------");
}
