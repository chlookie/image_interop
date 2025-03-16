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
	pub width: u32,

	/// The height of the image.
	pub height: u32,

	/// Add this stride to get to the next color channel for the same pixel.
	pub channel_stride: usize,

	/// Add this stride to get to the next pixel in the x-direction.
	pub x_stride: usize,

	/// Add this stride to get to the next pixel in the y-direction.
	pub y_stride: usize,
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
