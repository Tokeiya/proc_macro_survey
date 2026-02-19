use playground::prelude::Integer;
use syn::token::In;

pub enum EnumSample<'a, 'b, T: 'b>
where
	'a: 'b,
{
	TupleVariant((&'b T, i32), &'a str),
	NamedVariant {
		reference: &'a Integer,
		value: i32,
		tuple: (f32, f64),
	},
	UnitVariant,
	BareFn(fn(i32, i32) -> i32),
	Array([i32; 10]),
	Slice(&'a [T]),
}

pub struct NamedVariantStruct<'a, 'b> {
	pub reference: &'a Integer,
	pub value: &'b i32,
	pub tuple: &'b (f32, f64),
}

impl<'a, 'b, T: 'b> EnumSample<'a, 'b, T> {
	pub fn try_get_tuple_variant(&self) -> &(&'b T, i32) {
		match self {
			Self::TupleVariant(tup, _) => tup,
			_ => unreachable!(),
		}
	}

	pub fn try_get_named_variant(&self) -> NamedVariantStruct<'a, '_> {
		match self {
			EnumSample::NamedVariant {
				reference,
				value,
				tuple,
			} => NamedVariantStruct {
				reference,
				value,
				tuple,
			},
			_ => unreachable!(),
		}
	}
}

fn main() {}
