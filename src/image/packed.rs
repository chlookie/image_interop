use anyhow::ensure;

use crate::{Channels, ImageLayout, InterleavedImageLayout};

use super::{InterleavedLayout, InterleavedLayoutOrder};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the layout of a packed image, where each row/column of pixels is stored contiguously in memory, with no padding in between.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PackedInterleavedLayout {
	/// The width of the image.
	width: u32,

	/// The height of the image.
	height: u32,

	/// The storage order of the image, either column major or row major.
	order: InterleavedLayoutOrder,
}

impl PackedInterleavedLayout {
	/// Create a new packed layout.
	pub fn new(width: u32, height: u32, order: InterleavedLayoutOrder) -> Self {
		Self { width, height, order }
	}

	/// Get the layout storage order.
	pub fn order(&self) -> InterleavedLayoutOrder {
		self.order
	}
}

impl ImageLayout for PackedInterleavedLayout {
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

impl InterleavedImageLayout for PackedInterleavedLayout {
	fn pixel_index(&self, x: u32, y: u32) -> usize {
		match self.order {
			InterleavedLayoutOrder::RowMajor => y as usize * self.width as usize,
			InterleavedLayoutOrder::ColumnMajor => x as usize * self.height as usize,
		}
	}
}

impl TryFrom<InterleavedLayout> for PackedInterleavedLayout {
	type Error = ();

	fn try_from(value: InterleavedLayout) -> Result<Self, Self::Error> {
		ensure!(
			(value.width() == value.x_stride() && value.height() == value.y_stride())
				|| (value.width() == value.y_stride() && value.height() == value.x_stride())
		);

		Ok(Self {
			width: value.width(),
			height: value.height(),
			order: value.order(),
		})
	}
}
