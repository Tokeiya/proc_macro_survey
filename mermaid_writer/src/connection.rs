use crate::key::Key;
use crate::link::Link;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Connection<K> {
	pub source: K,
	pub target: K,
}

impl<K: Key, L: Link<K>> From<&L> for Connection<K> {
	fn from(value: &L) -> Self {
		Connection {
			source: value.source(),
			target: value.target(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::prelude::*;
	use crate::regular_link::RegularLink;
	#[test]
	fn from_ref() {
		let link = RegularLink::oneway(10, 20, LineShape::Normal, ArrowShape::Arrow, None);
		let fixture = Connection::from(&link);
		assert_eq!(fixture.source, 10);
		assert_eq!(fixture.target, 20);
	}
}
