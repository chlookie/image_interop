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

	pub fn is_well_formed(&self) -> bool {
		self.is_row_major() || self.is_column_major()
	}

	pub fn is_row_major(&self) -> bool {
		self.stride_y >= self.stride_x * self.width as usize
	}

	pub fn is_column_major(&self) -> bool {
		self.stride_x >= self.stride_y * self.height as usize
	}

	pub fn major_minor_strides(&self) -> Option<(usize, usize)> {
		if self.is_row_major() {
			Some((self.stride_y, self.stride_x))
		} else if self.is_column_major() {
			Some((self.stride_x, self.stride_y))
		} else {
			None
		}
	}

	pub fn major_minor_sidelengths(&self) -> Option<(u32, u32)> {
		if self.is_row_major() {
			Some((self.height, self.width))
		} else if self.is_column_major() {
			Some((self.width, self.height))
		} else {
			None
		}
	}

	pub fn total_visible_pixels(&self) -> u32 {
		self.width * self.height
	}

	pub fn total_padded_pixels(&self) -> Option<usize> {
		// Since we are using strides, can't just do width*height
		if self.is_row_major() {
			// stride_y >= stride_x
			Some(self.stride_y * self.height as usize)
		} else if self.is_column_major() {
			// stride_x >= stride_y
			Some(self.stride_x * self.width as usize)
		} else {
			// Cannot determine padded pixels
			None
		}
	}

	pub fn index(&self, x: u32, y: u32) -> usize {
		x as usize * self.stride_x + y as usize * self.stride_y
	}
}
