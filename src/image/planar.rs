use crate::{Channels, ImageLayout};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the layout of a planar image, where each color channel of a pixel is not necessarily stored contiguously in memory, but instead is on its own plane in a 3rd dimension.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PlanarLayout {
	/// The width of the image.
	width: u32,

	/// The height of the image.
	height: u32,

	/// Add this stride to get to the next color channel for the same pixel.
	channel_stride: usize,

	/// Add this stride to get to the next pixel in the x-direction.
	x_stride: usize,

	/// Add this stride to get to the next pixel in the y-direction.
	y_stride: usize,
}

impl PlanarLayout {
	/// Create a new planar layout.
	pub fn new(width: u32, height: u32, channel_stride: usize, x_stride: usize, y_stride: usize) -> Self {
		Self {
			width,
			height,
			channel_stride,
			x_stride,
			y_stride,
		}
	}

	pub fn channel_stride(&self) -> usize {
		self.channel_stride
	}

	pub fn x_stride(&self) -> usize {
		self.x_stride
	}

	pub fn y_stride(&self) -> usize {
		self.y_stride
	}
}

impl ImageLayout for PlanarLayout {
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}

	fn minimum_buffer_size(&self, channels: Channels) -> usize {
		(self.channel_stride * channels as usize)
			.max(self.x_stride * self.height as usize)
			.max(self.y_stride * self.width as usize)
	}

	fn color_channel_index(&self, x: u32, y: u32, channel: Channels) -> usize {
		channel * self.channel_stride + x as usize * self.x_stride + y as usize * self.y_stride
	}
}
