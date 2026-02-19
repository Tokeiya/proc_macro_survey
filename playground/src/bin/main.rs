use std::fmt::Display;

enum FnSample {
	Binary(fn(i32, i32) -> i32),
	Unary(fn(i32) -> i32),
}

fn main() {
	let a = FnSample::Binary(|a, b| a + b);
}
