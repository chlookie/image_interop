use anyhow::{Result, ensure};

use crate::{Channels, ImageLayout};

use super::InterleavedLayout;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the order in which the color channels are not stored contigously in memory.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LooseLayoutOrder {
	ChannelRowColumn,
	ChannelColumnRow,
	RowChannelColumn,
	RowColumnChannel,
	ColumnChannelRow,
	ColumnRowChannel,
}

impl LooseLayoutOrder {
	/// Compute the layout order from the strides and dimensions of the image.
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

/// Describes the layout of a loose-layout image, where the color channels are not necessarily stored contiguously in memory. This is a generalization of the interleaved layout and can be used for planar or other layouts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LooseLayout {
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

impl LooseLayout {
	/// Create a new loose layout.
	pub fn new(width: u32, height: u32, channel_stride: usize, x_stride: usize, y_stride: usize) -> Result<Self> {
		// Check that it is well-formed
		ensure!(channel_stride > 0, "Channel stride cannot be zero");
		ensure!(x_stride > 0, "X stride cannot be zero");
		ensure!(y_stride > 0, "Y stride cannot be zero");
		ensure!(
			LooseLayoutOrder::compute(channel_stride, x_stride, y_stride, width, height).is_some(),
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

	/// Get the channel stride.
	pub fn channel_stride(&self) -> usize {
		self.channel_stride
	}

	/// Get the x stride.
	pub fn x_stride(&self) -> usize {
		self.x_stride
	}

	/// Get the y stride.
	pub fn y_stride(&self) -> usize {
		self.y_stride
	}

	/// Get the layout storage order.
	pub fn order(&self) -> LooseLayoutOrder {
		LooseLayoutOrder::compute(
			self.channel_stride,
			self.x_stride,
			self.y_stride,
			self.width,
			self.height,
		)
		.expect("Computed layout should be valid since it was checked in the constructor")
	}
}

impl ImageLayout for LooseLayout {
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

	fn color_channel_index(&self, _channels: Channels, x: u32, y: u32, channel: Channels) -> usize {
		channel * self.channel_stride + x as usize * self.x_stride + y as usize * self.y_stride
	}
}

impl From<InterleavedLayout> for LooseLayout {
	fn from(value: InterleavedLayout) -> Self {
		Self {
			width: value.width(),
			height: value.height(),
			channel_stride: 1,
			x_stride: value.x_stride(),
			y_stride: value.y_stride(),
		}
	}
}
