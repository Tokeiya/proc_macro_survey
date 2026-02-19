use regex::Regex;

fn foo() {
	let reg = Regex::new("\\S+").unwrap();
	let cap = reg.find_iter("hello world hello rust");
	
	let mut a = "hello".to_string();
	for elem in cap {
		println!("{:?}", elem);
	}
}

fn main() {
	let reg = Regex::new(r#"\p{Lu}\P{Lu}*"#).unwrap();
	let cap = reg.find_iter("AHel90990loWorld");
	
	let mut str = String::new();
	
	for elem in cap {
		str.push_str(elem.as_str().to_lowercase().as_str());
		str.push_str("_");
		println!("{elem:?}")
	}
	
	str.pop().unwrap();
	
	println!("{str}")
}
