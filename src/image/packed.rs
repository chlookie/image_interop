use crate::{Channels, ImageLayout, InterleavedImageLayout};

use super::InterleavedLayoutOrder;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the layout of a packed image, where each row/column of pixels is stored contiguously in memory, with no padding in between.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PackedLayout {
	/// The width of the image.
	width: u32,

	/// The height of the image.
	height: u32,

	/// The storage order of the image, either column major or row major.
	order: InterleavedLayoutOrder,
}

impl PackedLayout {
	/// Create a new packed layout.
	pub fn new(width: u32, height: u32, order: InterleavedLayoutOrder) -> Self {
		Self { width, height, order }
	}

	pub fn order(&self) -> InterleavedLayoutOrder {
		self.order
	}
}

impl ImageLayout for PackedLayout {
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}

	fn minimum_buffer_size(&self, channels: Channels) -> usize {
		self.width as usize * self.height as usize * channels as usize
	}

	fn color_channel_index(&self, x: u32, y: u32, channel: Channels) -> usize {
		channel as usize + self.pixel_index(x, y)
	}
}

impl InterleavedImageLayout for PackedLayout {
	fn pixel_index(&self, x: u32, y: u32) -> usize {
		match self.order {
			InterleavedLayoutOrder::RowMajor => y as usize * self.width as usize,
			InterleavedLayoutOrder::ColumnMajor => x as usize * self.height as usize,
		}
	}
}
