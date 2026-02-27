use std::borrow::Cow;

pub fn convert(scr: &str) -> Cow<'_, str> {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn convert_from_snake() {
		let act = convert("snake_case_string");
		assert_eq!(act, "snake_case_string");
	}

	#[test]
	fn convert_from_lower_camel() {
		let act = convert("lowerCamelCaseString");
		assert_eq!(act, "lower_camel_case_string");
	}

	#[test]
	fn convert_from_upper_camel() {
		let act = convert("UpperCamelCaseString");
		assert_eq!(act, "upper_camel_case_string");
	}

	#[test]
	fn convert_from_non_ascii() {
		let act = convert("こんにちわ世界");
		assert_eq!(act, "こんにちわ世界");
	}
}
