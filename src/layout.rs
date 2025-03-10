use std::{cmp::Ordering, ops::BitOr};

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

	pub fn form(&self) -> Option<ImageForm> {
		todo!()
	}

	pub fn is_row_major(&self) -> bool {
		todo!()
	}

	pub fn is_column_major(&self) -> bool {
		todo!()
	}

	pub fn is_packed(&self) -> bool {
		todo!()
	}

	pub fn major_stride(&self) -> usize {
		if self.is_row_major() {
			self.stride_y
		} else {
			self.stride_x
		}
	}

	pub fn minor_stride(&self) -> usize {
		if self.is_row_major() {
			self.stride_x
		} else {
			self.stride_y
		}
	}

	pub fn major_sidelength(&self) -> u32 {
		if self.is_row_major() { self.height } else { self.width }
	}

	pub fn minor_sidelength(&self) -> u32 {
		if self.is_row_major() { self.width } else { self.height }
	}

	pub fn total_pixels(&self) -> u32 {
		self.width * self.height
	}

	pub fn max_samples(&self) -> usize {
		// Since we are using strides, can't just do width*height
		if self.is_row_major() {
			// stride_y > stride_x
			self.stride_y.checked_mul(self.height)
		} else if self.is_column_major() {
			// stride_x > stride_y
			self.stride_x.checked_mul(self.width)
		} else {
			// Layout is malformed
			None
		}
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
	/// The image has width > 0 and height > 0.
	WellFormed,

	/// The image doesn't have aliased pixel.
	/// In other words, for a given pixel in the buffer, there is at most one coordinate pair that maps to the pixel.
	/// Implies [`ImageForm::WellFormed`].
	Unaliased,

	/// The image's buffer is fully packed and utilized by the bounds of the image.
	/// Implies [`ImageForm::Unaliased`].
	Packed,

	/// The images bounds are width = 1 and height = 1.
	/// Implies [`ImageForm::Unaliased`].
	SinglePixel,

	/// The images bounds are width = 1 and height = 1, and the image is packed.
	/// Implies [`ImageForm::SinglePixel`] and [`ImageForm::Packed`].
	SinglePixelPacked,

	/// The image is in row-major form; i.e. rows are interated over first and pixels second.
	/// Implies [`ImageForm::Unaliased`].
	RowMajor,

	/// The image is in column-major form; i.e. columns are interated over first and pixels second.
	/// Implies [`ImageForm::Unaliased`].
	ColumnMajor,

	/// The image is in crowolumn-major form and packed.
	/// Implies [`ImageForm::RowMajor`].and [`ImageForm::Packed`].
	RowMajorPacked,

	/// The image is in column-major form and packed.
	/// Implies [`ImageForm::ColumnMajor`].and [`ImageForm::Packed`].
	ColumnMajorPacked,
}

impl ImageForm {
	fn implies(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Unaliased, Self::WellFormed) => true,

			(Self::Packed, Self::Unaliased) => true,

			(Self::SinglePixel, Self::Unaliased) => true,
			(Self::SinglePixelPacked, Self::SinglePixel) => true,
			(Self::SinglePixelPacked, Self::Packed) => true,

			(Self::RowMajor, Self::Unaliased) => true,
			(Self::RowMajorPacked, Self::RowMajor) => true,
			(Self::RowMajorPacked, Self::Packed) => true,

			(Self::ColumnMajor, Self::Unaliased) => true,
			(Self::ColumnMajorPacked, Self::ColumnMajor) => true,
			(Self::ColumnMajorPacked, Self::Packed) => true,

			_ => false,
		}
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
