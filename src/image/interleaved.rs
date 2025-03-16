use crate::{Channels, ImageLayout, InterleavedImageLayout};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the layout of an interleaved image, where as opposed to planar, all channels are stored contiguously in memory.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InterleavedLayout {
	/// The width of the image.
	width: u32,

	/// The height of the image.
	height: u32,

	/// Add this stride to get to the next pixel in the x-direction.
	x_stride: usize,

	/// Add this stride to get to the next pixel in the y-direction.
	y_stride: usize,
}

impl InterleavedLayout {
	/// Create a new interleaved layout.
	pub fn new(width: u32, height: u32, x_stride: usize, y_stride: usize) -> Self {
		Self {
			width,
			height,
			x_stride,
			y_stride,
		}
	}

	pub fn x_stride(&self) -> usize {
		self.x_stride
	}

	pub fn y_stride(&self) -> usize {
		self.y_stride
	}
}

impl ImageLayout for InterleavedLayout {
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}

	fn minimum_buffer_size(&self, channels: Channels) -> usize {
		(self.x_stride * self.height as usize).max(self.y_stride * self.width as usize) * channels as usize
	}

	fn color_channel_index(&self, x: u32, y: u32, channel: Channels) -> usize {
		channel as usize + self.pixel_index(x, y)
	}
}

impl InterleavedImageLayout for InterleavedLayout {
	fn pixel_index(&self, x: u32, y: u32) -> usize {
		x as usize * self.x_stride + y as usize * self.y_stride
	}
}

impl InterleavedLayout {
	fn is_row_major(&self) -> bool {
		self.y_stride >= self.x_stride * self.width as usize
	}

	fn is_column_major(&self) -> bool {
		self.x_stride >= self.y_stride * self.height as usize
	}
}
