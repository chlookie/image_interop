use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageForm {
	/// The image has width == 0 and/or height == 0 or has aliased pixels.
	Malformed,

	/// The image has width > 0 and height > 0 and doesn't have aliased pixel.
	/// In other words, for a given pixel in the buffer, there is at most one coordinate pair that maps to the pixel.
	WellFormed,

	/// The image is in row-major form; i.e. rows are interated over first and pixels second.
	/// Implies [`ImageForm::WellFormed`].
	RowMajor,

	/// The image is in column-major form; i.e. columns are interated over first and pixels second.
	/// Implies [`ImageForm::WellFormed`].
	ColumnMajor,

	/// The image has width = 1 and height = 1.
	/// Implies [`ImageForm::WellFormed`].
	SinglePixel,

	/// The image's buffer is fully packed and utilized by the bounds of the image.
	/// Implies [`ImageForm::WellFormed`].
	Packed,

	/// The image is in row-major form and packed.
	/// Implies [`ImageForm::RowMajor`] and [`ImageForm::Packed`].
	RowMajorPacked,

	/// The image is in column-major form and packed.
	/// Implies [`ImageForm::ColumnMajor`] and [`ImageForm::Packed`].
	ColumnMajorPacked,
}

impl ImageForm {
	pub fn implies(&self, other: &Self) -> bool {
		// TODO make constant instead of recursive?
		match self {
			_ if self == other => true,

			Self::RowMajor => Self::WellFormed.implies(other),
			Self::ColumnMajor => Self::WellFormed.implies(other),

			Self::SinglePixel => Self::WellFormed.implies(other),

			Self::Packed => Self::WellFormed.implies(other),
			Self::RowMajorPacked => Self::Packed.implies(other) || Self::RowMajor.implies(other),
			Self::ColumnMajorPacked => Self::Packed.implies(other) || Self::ColumnMajor.implies(other),

			_ => false,
		}
	}

	pub fn is_mal_formed(&self) -> bool {
		self <= &Self::Malformed
	}

	pub fn is_well_formed(&self) -> bool {
		self >= &Self::WellFormed
	}

	pub fn is_row_major(&self) -> bool {
		self >= &Self::RowMajor
	}

	pub fn is_column_major(&self) -> bool {
		self >= &Self::ColumnMajor
	}

	pub fn is_single_pixel(&self) -> bool {
		self >= &Self::SinglePixel
	}

	pub fn is_packed(&self) -> bool {
		self >= &Self::Packed
	}

	pub fn is_row_major_packed(&self) -> bool {
		self >= &Self::RowMajorPacked
	}

	pub fn is_column_major_packed(&self) -> bool {
		self >= &Self::ColumnMajorPacked
	}
}

impl PartialOrd for ImageForm {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (*self, *other) {
			(x, y) if x == y => Some(Ordering::Equal),
			(x, y) if x.implies(&y) => Some(Ordering::Greater),
			(x, y) if y.implies(&x) => Some(Ordering::Less),
			_ => None,
		}
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_image_form_partial_ord() {
		// TODO: Remake tests
		// 	assert!(ImageForm::WellFormed >= ImageForm::WellFormed);
		// 	assert!(ImageForm::Packed <= ImageForm::Packed);
		// 	assert!(ImageForm::Malformed <= ImageForm::Malformed);

		// 	assert!(ImageForm::Packed >= ImageForm::WellFormed);
		// 	assert!(ImageForm::SinglePixel >= ImageForm::);

		// 	assert!(ImageForm::Packed >= ImageForm::WellFormed);
		// 	assert!(ImageForm::Packed >= ImageForm::WellFormed);

		// 	assert!(ImageForm::Packed <= ImageForm::SinglePixelPacked);
		// 	assert!(ImageForm::SinglePixel <= ImageForm::SinglePixelPacked);
		// 	assert!(ImageForm::WellFormed <= ImageForm::SinglePixelPacked);

		// 	assert!(!(ImageForm::SinglePixel < ImageForm::SinglePixel));
		// 	assert!(!(ImageForm::Packed <= ImageForm::WellFormed));
		// 	assert!(!(ImageForm::Packed <= ImageForm::WellFormed));
	}
}
