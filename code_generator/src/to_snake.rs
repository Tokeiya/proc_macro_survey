use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

static IS_SNAKE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z]+(_[a-z0-9]+)*$").unwrap());
static CONTAIN_OTHER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^(A-Za-z0-9_)]+$").unwrap());
pub fn convert(scr: &str) -> Cow<'_, str> {
	if CONTAIN_OTHER.is_match(scr) || IS_SNAKE.is_match(scr) || scr.is_empty() {
		Cow::from(scr)
	} else {
		let mut buff = String::new();
		let mut iter = scr.chars();

		buff.push(iter.next().unwrap().to_ascii_lowercase());

		for c in iter {
			if c.is_ascii_uppercase() {
				buff.push('_');
				buff.push(c.to_ascii_lowercase());
			} else {
				buff.push(c);
			}
		}

		Cow::from(buff)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn foo() {
		let a = IS_SNAKE.is_match("Aaa");
		dbg!(a);
	}

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
