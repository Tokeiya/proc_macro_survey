#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
	Visible(Shape),
	Invisible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
	Normal,
	Dotted,
	Thick,
}
