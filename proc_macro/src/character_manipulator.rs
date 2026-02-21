use std::ascii::AsciiExt;

pub fn to_lower_snake(input: &str, output: &mut String) {
	let mut iter = input.chars();

	output.push(iter.next().unwrap().to_ascii_lowercase());

	while let Some(c) = iter.next() {
		if c.is_ascii_uppercase() {
			output.push('_');
			output.push(c.to_ascii_lowercase());
		} else {
			output.push(c);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_snake() {
		let mut actual = String::new();
		to_lower_snake("HelloWorld", &mut actual);
		assert_eq!(actual, "hello_world");

		actual.clear();

		to_lower_snake("hello_world", &mut actual);
		assert_eq!(actual, "hello_world");
	}
}
