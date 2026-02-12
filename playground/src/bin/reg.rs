use regex::Regex;

fn main() {
	let reg = Regex::new("\\S+").unwrap();
	let cap = reg.find_iter("hello world hello rust");

	for elem in cap {
		println!("{:?}", elem);
	}
}
