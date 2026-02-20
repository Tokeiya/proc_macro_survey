use playground::visitor::visit_checker::Visit as VisitCheck;
use syn::visit::Visit;
use syn::{Item, ItemEnum, parse_quote};
fn main() {
	let mut visit = VisitCheck;

	let input: Item = parse_quote! {
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
	};

	visit.visit_item(&input);
}
