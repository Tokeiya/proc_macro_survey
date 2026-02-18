use crate::key::Key;
use crate::link::Link;
use std::cmp::Ordering;

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

impl<K: PartialOrd> PartialOrd for Connection<K> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl<K: Ord> Ord for Connection<K> {
	fn cmp(&self, other: &Self) -> Ordering {
		todo!()
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

	#[test]
	fn partial_ord() {
		let a = f64::NAN;
		let b = f64::NAN;

		let a = a.total_cmp(&b);

		// let pivot = Connection {
		// 	source: 50,
		// 	target: 42,
		// };
		// let act = pivot
		// 	.partial_cmp(&Connection {
		// 		source: 50,
		// 		target: 42,
		// 	})
		// 	.unwrap();
		// assert!(matches!(act, Ordering::Equal));
	}
}
