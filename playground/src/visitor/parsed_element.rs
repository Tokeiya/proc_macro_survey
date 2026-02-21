use quote::{ToTokens, quote};
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize)]
pub struct Element {
	name: String,
	contents: String,
	children: Vec<Rc<RefCell<Element>>>,
}

impl Element {
	pub fn from_token_stream(name: &str, contents: impl ToTokens) -> Self {
		let quoted = quote! {#contents};
		let str = quoted
			.to_string()
			.replace("\"", "\\\"")
			.replace("#", "#35;");

		Self {
			name: name.to_string(),
			contents: str,
			children: Vec::new(),
		}
	}

	pub fn from_string(name: String, contents: String) -> Self {
		Self {
			name,
			contents,
			children: Vec::new(),
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn content(&self) -> &str {
		&self.contents
	}

	pub fn children(&self) -> &[Rc<RefCell<Element>>] {
		&self.children
	}

	pub fn add_child(&mut self, child: Rc<RefCell<Element>>) {
		self.children.push(child);
	}
}
