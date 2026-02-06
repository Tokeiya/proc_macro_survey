use proc_macro::add_check;

#[add_check]
pub enum Foo {
	Hoge,
	Piyo,
}

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_key_only() {
		let a = Foo::Hoge;
		assert!(a.is_hoge());
	}
}
