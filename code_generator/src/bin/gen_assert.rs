use code_generator::gen_assert;

enum Foo {
	Hoge,
	Piyo,
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
