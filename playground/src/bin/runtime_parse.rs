use syn::parse_str;

const INPUT: &str = r##"
		#[cfg(feature = "dummy_a")]
		#[cfg(feature = "dummy_a")]
		pub enum EnumSample<'a, 'b, T: 'b>
		where
			'a: 'b,
		{
			#[cfg(feature = "dummy_b")]
			#[doc = "document"]
			TupleVariant(
				#[cfg(feature = "dummy_b")] (&'b T, i32),
				#[cfg(feature = "dummy_b")] &'a str,
			),
			#[cfg(feature = "dummy_c")]
			NamedVariant {
				#[cfg(feature = "dummy_c")]
				reference: &'a Integer,
				value: i32,
				#[doc = "foo"]
				tuple: (std::f32, f64),
			},
			#[cfg(feature = "dummy_d")]
			UnitVariant,
			BareFnA(fn(i32, i32) -> i32),
			BareFnB(fn()->()),
			BareFnC(fn()),
			BareFnD(extern "C" fn(i32,i32)->i32),
			Array([i32; 10]),
			Slice(&'a [T]),
		}
"##;

fn main() {
	let parsed = parse_str::<syn::Item>(INPUT);

	if let Err(e) = parsed {
		dbg!(e);
	}
}
