use code_generator::gen_assert;

enum Foo {
	Hoge,
	Piyo,
}

#[cfg(test)]
impl Foo {
	pub fn assert_hoge(&self) {
		match self {
			Foo::Hoge => {}
			_ => unreachable!(),
		}
	}
	pub fn assert_piyo(&self) {
		match self {
			Foo::Piyo => {}
			_ => unreachable!(),
		}
	}
}

fn main() {
	const SCR: &str = r##"
	enum Foo {
	Hoge,
	Piyo,
}
	"##;

	let a = gen_assert::gen_assert(SCR).unwrap();

	println!("{}", a);
}
