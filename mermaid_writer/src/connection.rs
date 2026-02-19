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
		Some(
			match (
				self.source.partial_cmp(&other.source).unwrap(),
				self.target.partial_cmp(&other.target).unwrap(),
			) {
				(Ordering::Greater, _) => Ordering::Greater,
				(Ordering::Less, _) => Ordering::Less,
				(Ordering::Equal, Ordering::Greater) => Ordering::Greater,
				(Ordering::Equal, Ordering::Less) => Ordering::Less,
				(Ordering::Equal, Ordering::Equal) => Ordering::Equal,
			},
		)
	}
}

impl<K: Ord> Ord for Connection<K> {
	fn cmp(&self, other: &Self) -> Ordering {
		match (
			self.source.cmp(&other.source),
			self.target.cmp(&other.target),
		) {
			(Ordering::Greater, _) => Ordering::Greater,
			(Ordering::Less, _) => Ordering::Less,
			(Ordering::Equal, Ordering::Greater) => Ordering::Greater,
			(Ordering::Equal, Ordering::Less) => Ordering::Less,
			(Ordering::Equal, Ordering::Equal) => Ordering::Equal,
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

	#[test]
	fn partial_ord() {
		let pivot = Connection {
			source: 50,
			target: 42,
		};

		let act = pivot
			.partial_cmp(&Connection {
				source: 50,
				target: 42,
			})
			.unwrap();
		assert!(matches!(act, Ordering::Equal));

		let act = pivot
			.partial_cmp(&Connection {
				source: 50,
				target: 41,
			})
			.unwrap();
		assert!(matches!(act, Ordering::Greater));

		let act = pivot.partial_cmp(&Connection {
			source: 50,
			target: 43,
		});
		assert!(matches!(act, Some(Ordering::Less)));

		let act = pivot.partial_cmp(&Connection {
			source: 49,
			target: 42,
		});
		assert!(matches!(act, Some(Ordering::Greater)));

		let act = pivot.partial_cmp(&Connection {
			source: 49,
			target: 43,
		});
		assert!(matches!(act, Some(Ordering::Greater)));

		let act = pivot.partial_cmp(&Connection {
			source: 51,
			target: 43,
		});
		assert!(matches!(act, Some(Ordering::Less)));
	}

	#[test]
	fn ord() {
		let pivot = Connection {
			source: 50,
			target: 42,
		};

		let act = pivot.cmp(&pivot);
		assert!(matches!(act, Ordering::Equal));

		let act = pivot.cmp(&Connection {
			source: 50,
			target: 41,
		});
		assert!(matches!(act, Ordering::Greater));

		let act = pivot.cmp(&Connection {
			source: 50,
			target: 43,
		});
		assert!(matches!(act, Ordering::Less));

		let act = pivot.cmp(&Connection {
			source: 49,
			target: 42,
		});

		assert!(matches!(act, Ordering::Greater));

		let act = pivot.cmp(&Connection {
			source: 49,
			target: 43,
		});

		assert!(matches!(act, Ordering::Greater));

		let act = pivot.cmp(&Connection {
			source: 51,
			target: 43,
		});

		assert!(matches!(act, Ordering::Less));
	}
}
