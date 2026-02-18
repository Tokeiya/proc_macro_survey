use rand::prelude::*;
use rand::seq::SliceRandom;
fn main() {
	let mut vec: Vec<(i32, i32)> = Vec::new();

	for i in 0..5 {
		for j in 0..5 {
			vec.push((i, j))
		}
	}

	vec.shuffle(&mut rand::rng());

	for (idx, (i, j)) in vec.iter().enumerate() {
		println!("Index: {}, Value: ({}, {})", idx, i, j);
	}

	println!("sort");
	vec.sort();

	for (idx, (i, j)) in vec.iter().enumerate() {
		println!("Index: {}, Value: ({}, {})", idx, i, j);
	}
}
