use crate::{Channels, ImageLayout, InterleavedImageLayout};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PackedLayoutOrder {
	RowMajor,
	ColumnMajor,
}

/// Describes the layout of a packed image, where each row/column of pixels is stored contiguously in memory, with no padding in between.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PackedLayout {
	/// The width of the image.
	pub width: u32,

	/// The height of the image.
	pub height: u32,

	/// The storage order of the image, either column major or row major.
	pub order: PackedLayoutOrder,
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
			PackedLayoutOrder::RowMajor => y as usize * self.width as usize,
			PackedLayoutOrder::ColumnMajor => x as usize * self.height as usize,
		}
	}
}
