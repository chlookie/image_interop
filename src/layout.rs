use std::cmp::Ordering;

use crate::{Channels, ImageForm};

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

	pub fn incomplete_form(&self) -> ImageForm {
		if self.width == 0 || self.height == 0 {
			ImageForm::Malformed
		} else if self.width == 1 && self.height == 1 {
			ImageForm::SinglePixel
		} else if self.stride_y >= self.stride_x * self.width as usize {
			ImageForm::RowMajor
		} else if self.stride_x >= self.stride_y * self.height as usize {
			ImageForm::ColumnMajor
		} else {
			ImageForm::Malformed
		}
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
