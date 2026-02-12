use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
pub struct KeyValue(i32, String);
fn main() {
	let mut hash = HashSet::<KeyValue>::new();
	hash.insert(KeyValue(1, "hello".to_string()));
}
