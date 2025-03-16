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
	pub width: u32,

	/// The height of the image.
	pub height: u32,

	/// Add this stride to get to the next pixel in the x-direction.
	pub stride_x: usize,

	/// Add this stride to get to the next pixel in the y-direction.
	pub stride_y: usize,
}

impl ImageLayout for InterleavedLayout {
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}

	fn minimum_buffer_size(&self, channels: Channels) -> usize {
		(self.stride_x * self.height as usize).max(self.stride_y * self.width as usize) * channels as usize
	}

	fn color_channel_index(&self, x: u32, y: u32, channel: Channels) -> usize {
		channel as usize + self.pixel_index(x, y)
	}
}

impl InterleavedImageLayout for InterleavedLayout {
	fn pixel_index(&self, x: u32, y: u32) -> usize {
		x as usize * self.stride_x + y as usize * self.stride_y
	}
}

impl InterleavedLayout {
	fn is_row_major(&self) -> bool {
		self.stride_y >= self.stride_x * self.width as usize
	}

	fn is_column_major(&self) -> bool {
		self.stride_x >= self.stride_y * self.height as usize
	}
}
