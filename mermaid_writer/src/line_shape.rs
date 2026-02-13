#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineStyle {
	Visible(Shape),
	Invisible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
	Normal,
	Dotted,
	Thick,
}
