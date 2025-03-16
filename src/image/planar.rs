use anyhow::ensure;

use crate::{Channels, ImageLayout};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the order in which the color channels are stored in a channel-separated (i.e. not necessarily interleaved or planar) image.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChannelSeparatedLayoutOrder {
	ChannelRowColumn,
	ChannelColumnRow,
	RowChannelColumn,
	RowColumnChannel,
	ColumnChannelRow,
	ColumnRowChannel,
}

impl ChannelSeparatedLayoutOrder {
	pub fn compute(channel_stride: usize, x_stride: usize, y_stride: usize, width: u32, height: u32) -> Option<Self> {
		if channel_stride >= x_stride * height as usize && x_stride >= y_stride * width as usize {
			Some(Self::ChannelRowColumn)
		} else if channel_stride >= y_stride * width as usize && y_stride >= x_stride * height as usize {
			Some(Self::ChannelColumnRow)
		} else if x_stride >= channel_stride * height as usize && channel_stride >= y_stride * width as usize {
			Some(Self::RowChannelColumn)
		} else if x_stride >= y_stride * width as usize && y_stride >= channel_stride * height as usize {
			Some(Self::RowColumnChannel)
		} else if y_stride >= channel_stride * width as usize && channel_stride >= x_stride * height as usize {
			Some(Self::ColumnChannelRow)
		} else if y_stride >= x_stride * height as usize && x_stride >= channel_stride * width as usize {
			Some(Self::ColumnRowChannel)
		} else {
			None
		}
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the layout of a planar image, where each color channel of a pixel is not necessarily stored contiguously in memory, but instead is on its own plane in a 3rd dimension.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChannelSeparatedLayout {
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

impl ChannelSeparatedLayout {
	/// Create a new planar layout.
	pub fn new(width: u32, height: u32, channel_stride: usize, x_stride: usize, y_stride: usize) -> Result<Self> {
		// Check that it is well-formed
		ensure!(channel_stride > 0, "Channel stride cannot be zero");
		ensure!(x_stride > 0, "X stride cannot be zero");
		ensure!(y_stride > 0, "Y stride cannot be zero");
		ensure!(
			ChannelSeparatedLayoutOrder::compute(channel_stride, x_stride, y_stride, width, height).is_some(),
			"Invalid strides"
		);

		Ok(Self {
			width,
			height,
			channel_stride,
			x_stride,
			y_stride,
		})
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

	fn order(&self) -> ChannelSeparatedLayoutOrder {
		ChannelSeparatedLayoutOrder::compute(
			self.channel_stride,
			self.x_stride,
			self.y_stride,
			self.width,
			self.height,
		)
		.expect("Computed layout should be valid since it was checked in the constructor")
	}
}

impl ImageLayout for ChannelSeparatedLayout {
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
