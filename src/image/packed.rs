use anyhow::{Result, ensure};

use crate::{Channels, ImageLayout, InterleavedImageLayout};

use super::{InterleavedLayout, InterleavedLayoutOrder};

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
	pub fn new(width: u32, height: u32, order: InterleavedLayoutOrder) -> Result<Self> {
		ensure!(width > 0, "Width cannot be zero");
		ensure!(height > 0, "Height cannot be zero");

		Ok(Self { width, height, order })
	}

	pub fn major_minor_sidelengths(&self) -> (u32, u32) {
		if self.is_row_major() {
			(self.width, self.height)
		} else {
			(self.height, self.width)
		}
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

	fn color_channel_index(&self, channels: Channels, x: u32, y: u32, channel: Channels) -> usize {
		channel as usize + self.pixel_index(channels, x, y)
	}
}

impl InterleavedImageLayout for PackedLayout {
	fn pixel_index(&self, channels: Channels, x: u32, y: u32) -> usize {
		match self.order {
			InterleavedLayoutOrder::RowMajor => x as usize + y as usize * self.width as usize * channels,
			InterleavedLayoutOrder::ColumnMajor => y as usize + x as usize * self.height as usize * channels,
		}
	}

	fn order(&self) -> InterleavedLayoutOrder {
		self.order
	}
}

impl TryFrom<InterleavedLayout> for PackedLayout {
	type Error = anyhow::Error;

	fn try_from(value: InterleavedLayout) -> Result<Self, Self::Error> {
		ensure!(
			(value.width() == value.x_stride().try_into()? && value.height() == value.y_stride().try_into()?)
				|| (value.width() == value.y_stride().try_into()? && value.height() == value.x_stride().try_into()?),
		);

		Ok(Self {
			width: value.width(),
			height: value.height(),
			order: value.order(),
		})
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_constructor() {
		let layout = PackedLayout::new(10, 20, InterleavedLayoutOrder::RowMajor).unwrap();
		assert_eq!(layout.width(), 10);
		assert_eq!(layout.height(), 20);
		assert_eq!(layout.order(), InterleavedLayoutOrder::RowMajor);

		let layout = PackedLayout::new(90, 5, InterleavedLayoutOrder::ColumnMajor).unwrap();
		assert_eq!(layout.width(), 90);
		assert_eq!(layout.height(), 5);
		assert_eq!(layout.order(), InterleavedLayoutOrder::ColumnMajor);

		assert!(PackedLayout::new(0, 20, InterleavedLayoutOrder::RowMajor).is_err());

		assert!(PackedLayout::new(10, 0, InterleavedLayoutOrder::RowMajor).is_err());
	}

	#[test]
	fn test_minimum_buffer_size() {
		let layout = PackedLayout::new(10, 20, InterleavedLayoutOrder::RowMajor).unwrap();
		assert_eq!(layout.minimum_buffer_size(3), 10 * 20 * 3);

		let layout = PackedLayout::new(90, 5, InterleavedLayoutOrder::ColumnMajor).unwrap();
		assert_eq!(layout.minimum_buffer_size(4), 90 * 5 * 4);
	}

	#[test]
	fn test_color_channel_index() {
		let layout = PackedLayout::new(10, 20, InterleavedLayoutOrder::RowMajor).unwrap();
		assert_eq!(layout.color_channel_index(3, 5, 12, 0), 5 + 12 * 10 * 3);

		let layout = PackedLayout::new(90, 5, InterleavedLayoutOrder::ColumnMajor).unwrap();
		assert_eq!(layout.color_channel_index(4, 7, 10, 1), 10 + 7 * 5 * 4 + 1);
	}
}
