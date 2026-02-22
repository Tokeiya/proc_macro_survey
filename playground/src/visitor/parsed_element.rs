use quote::{ToTokens, quote};
use serde::Serialize;
use std::cell::RefCell;
use std::io::Write;
use std::process::{Command, Stdio};
use std::rc::Rc;
fn format_code(code: &str) -> String {
	let mut child = Command::new("rustfmt")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.expect("Failed to spawn rustfmt process");

	let mut stdin = child.stdin.take().expect("Failed to open stdin");
	stdin
		.write_all(code.as_bytes())
		.expect("Failed to write to stdin");

	drop(stdin); // Close stdin to signal EOF to rustfmt

	let output = child.wait_with_output().expect("Failed to read output");

	let ret = String::from_utf8_lossy(&output.stdout)
		.to_string()
		.replace("\t", "    ")
		.trim()
		.to_string();
	ret
}

#[derive(Serialize)]
pub struct Element {
	name: String,
	contents: String,
	children: Vec<Rc<RefCell<Element>>>,
}

impl Element {
	pub fn from_token_stream(name: &str, contents: impl ToTokens, format: bool) -> Self {
		let quoted = quote! {#contents};
		let str = quoted.to_string();

		if format {
			Self {
				name: name.to_string(),
				contents: format_code(&str),
				children: Vec::new(),
			}
		} else {
			Self {
				name: name.to_string(),
				contents: str,
				children: Vec::new(),
			}
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
