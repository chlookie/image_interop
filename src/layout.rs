use std::cmp::Ordering;

use crate::Channels;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ImageLayout {
	/// The width of the image.
	pub width: u32,

	/// The height of the image.
	pub height: u32,

	/// Add this stride to get to the next pixel in the x-direction.
	pub stride_x: usize,

	/// Add this stride to get to the next pixel in the y-direction.
	pub stride_y: usize,
}

impl ImageLayout {
	pub fn new(width: u32, height: u32, stride_x: usize, stride_y: usize) -> Self {
		ImageLayout {
			width,
			height,
			stride_x,
			stride_y,
		}
	}

	pub fn row_major_packed(channels: Channels, width: u32, height: u32) -> Self {
		let stride_x = channels;
		let stride_y = channels * width as usize;

		Self::new(width, height, stride_x, stride_y)
	}

	pub fn column_major_packed(channels: Channels, width: u32, height: u32) -> Self {
		let stride_x = channels * height as usize;
		let stride_y = channels;

		Self::new(width, height, stride_x, stride_y)
	}

	pub fn form(&self, channels: Channels) -> ImageForm {
		todo!()
		// match (self.width, self.height, self.stride_x, self.stride_y) {
		// 	(0, 0, _, _) => ImageForm::Empty,
		// 	(1, 1, sx, sy) if sx == channels && sy == => ImageForm::SinglePixel,
		// 	(1, 1, _, _) => ImageForm::SinglePixel,
		// 	(1, _, _, _) => ImageForm::VerticalStripe,
		// 	(_, 1, _, _) => ImageForm::HorizontalStripe,
		// 	_ => {}
		// };

		// if self.width == 0 || self.height == 0 {
		// 	ImageForm::Empty
		// } else if self.width == 0 {

		// } else if
	}

	// pub fn major_stride(&self) -> usize {
	// 	if self.form().is_row_major() {
	// 		self.stride_y
	// 	} else {
	// 		self.stride_x
	// 	}
	// }

	// pub fn minor_stride(&self) -> usize {
	// 	if self.form().is_row_major() {
	// 		self.stride_x
	// 	} else {
	// 		self.stride_y
	// 	}
	// }

	// pub fn major_sidelength(&self) -> u32 {
	// 	if self.form().is_row_major() { self.height } else { self.width }
	// }

	// pub fn minor_sidelength(&self) -> u32 {
	// 	if self.form().is_row_major() { self.width } else { self.height }
	// }

	pub fn total_pixels(&self) -> u32 {
		self.width * self.height
	}

	pub fn max_samples(&self) -> usize {
		todo!()
		// // Since we are using strides, can't just do width*height
		// if self.is_row_major() {
		// 	// stride_y > stride_x
		// 	self.stride_y.checked_mul(self.height)
		// } else if self.is_column_major() {
		// 	// stride_x > stride_y
		// 	self.stride_x.checked_mul(self.width)
		// } else {
		// 	// Layout is malformed
		// 	None
		// }
	}

	pub fn index(&self, x: u32, y: u32) -> usize {
		x as usize * self.stride_x + y as usize * self.stride_y
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageForm {
	/// The image has width == 0 and/or height == 0 or has aliased pixels.
	Malformed,

	/// The image has width > 0 and height > 0.
	NonEmpty,

	/// The image doesn't have aliased pixel.
	/// In other words, for a given pixel in the buffer, there is at most one coordinate pair that maps to the pixel.
	/// Implies [`ImageForm::NonEmpty`].
	Unaliased,

	/// The image is in row-major form; i.e. rows are interated over first and pixels second.
	/// Implies [`ImageForm::Unaliased`].
	RowMajor,

	/// The image is in column-major form; i.e. columns are interated over first and pixels second.
	/// Implies [`ImageForm::Unaliased`].
	ColumnMajor,

	/// The image has width = 1.
	/// Implies [`ImageForm::ColumnMajor`].
	VerticalStripe,

	/// The image has height = 1.
	/// Implies [`ImageForm::RowMajor`].
	HorizontalStripe,

	/// The image has width = 1 and height = 1.
	/// Implies [`ImageForm::VerticalStripe`] and [`ImageForm::HorizontalStripe`].
	SinglePixel,

	/// The image's buffer is fully packed and utilized by the bounds of the image.
	/// Implies [`ImageForm::Unaliased`].
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

			Self::Unaliased => Self::NonEmpty.implies(other),

			Self::RowMajor => Self::Unaliased.implies(other),
			Self::ColumnMajor => Self::Unaliased.implies(other),

			Self::VerticalStripe => Self::ColumnMajor.implies(other),
			Self::HorizontalStripe => Self::RowMajor.implies(other),
			Self::SinglePixel => Self::VerticalStripe.implies(other) || Self::HorizontalStripe.implies(other),

			Self::Packed => Self::Unaliased.implies(other),
			Self::RowMajorPacked => Self::Packed.implies(other) || Self::RowMajor.implies(other),
			Self::ColumnMajorPacked => Self::Packed.implies(other) || Self::ColumnMajor.implies(other),

			_ => false,
		}
	}

	pub fn is_non_empty(&self) -> bool {
		self >= &Self::NonEmpty
	}

	pub fn is_unaliased(&self) -> bool {
		self >= &Self::Unaliased
	}

	pub fn is_row_major(&self) -> bool {
		self >= &Self::RowMajor
	}

	pub fn is_column_major(&self) -> bool {
		self >= &Self::ColumnMajor
	}

	pub fn is_vertical_stripe(&self) -> bool {
		self >= &Self::VerticalStripe
	}

	pub fn is_horizontal_stripe(&self) -> bool {
		self >= &Self::HorizontalStripe
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
		assert!(ImageForm::NonEmpty >= ImageForm::NonEmpty);
		assert!(ImageForm::Packed <= ImageForm::Packed);

		assert!(ImageForm::Packed >= ImageForm::Unaliased);
		assert!(ImageForm::Packed >= ImageForm::NonEmpty);

		assert!(ImageForm::Packed >= ImageForm::Unaliased);
		assert!(ImageForm::Packed >= ImageForm::NonEmpty);

		assert!(ImageForm::Packed <= ImageForm::SinglePixelPacked);
		assert!(ImageForm::SinglePixel <= ImageForm::SinglePixelPacked);
		assert!(ImageForm::NonEmpty <= ImageForm::SinglePixelPacked);

		assert!(!(ImageForm::SinglePixel < ImageForm::SinglePixel));
		assert!(!(ImageForm::Packed <= ImageForm::Unaliased));
		assert!(!(ImageForm::Packed <= ImageForm::NonEmpty));
	}
}
