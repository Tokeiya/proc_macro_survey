use std::io::Write;
use std::process::{Command, Stdio};
fn main() {
	const INPUT: &str = "# [cfg (feature = \"dummy_a\")] # [cfg (feature = \"dummy_a\")] pub enum EnumSample < 'a , 'b , T : 'b > where 'a : 'b , { # [cfg (feature = \"dummy_b\")] # [doc = \"document\"] TupleVariant (# [cfg (feature = \"dummy_b\")] (& 'b T , i32) , # [cfg (feature = \"dummy_b\")] & 'a str ,) , # [cfg (feature = \"dummy_c\")] NamedVariant { # [cfg (feature = \"dummy_c\")] reference : & 'a Integer , value : i32 , # [doc = \"foo\"] tuple : (std :: f32 , f64) , } , # [cfg (feature = \"dummy_d\")] UnitVariant , BareFnA (fn (i32 , i32) -> i32) , BareFnB (fn () -> ()) , BareFnC (fn ()) , BareFnD (extern \"C\" fn (i32 , i32) -> i32) , Array ([i32 ; 10]) , Slice (& 'a [T]) , }";

	let mut child = Command::new("rustfmt")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.expect("Failed to spawn rustfmt process");

	let mut stdin = child.stdin.take().expect("Failed to open stdin");
	stdin
		.write_all(INPUT.as_bytes())
		.expect("Failed to write to stdin");

	drop(stdin); // Close stdin to signal EOF to rustfmt

	let output = child.wait_with_output().expect("Failed to read output");

	println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn foo(opt: Option<i32>) {}
