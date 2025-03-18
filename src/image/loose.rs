use std::ops::{Deref, DerefMut};

use anyhow::{Result, ensure};

use crate::{ChannelShrinkable, Channels, Color, ImageLayout, ImageView, ImageViewMut};

use super::{GenericImage, InterleavedLayout, PackedLayout};

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
	/// The number of color channels in the image.
	channels: Channels,

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
	pub fn new(
		channels: Channels,
		width: u32,
		height: u32,
		channel_stride: usize,
		x_stride: usize,
		y_stride: usize,
	) -> Result<Self> {
		// Check that it is well-formed
		ensure!(channels > 0, "Number of channels cannot be zero");
		ensure!(width > 0, "Width cannot be zero");
		ensure!(height > 0, "Height cannot be zero");
		ensure!(channel_stride > 0, "Channel stride cannot be zero");
		ensure!(x_stride > 0, "X stride cannot be zero");
		ensure!(y_stride > 0, "Y stride cannot be zero");
		ensure!(
			LooseLayoutOrder::compute(channel_stride, x_stride, y_stride, width, height).is_some(),
			"Invalid strides"
		);

		Ok(Self {
			channels,
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
	fn channels(&self) -> Channels {
		self.channels
	}

	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}

	fn minimum_buffer_size(&self) -> usize {
		(self.channel_stride * self.channels as usize)
			.max(self.x_stride * self.height as usize)
			.max(self.y_stride * self.width as usize)
	}

	fn component_index_unchecked(&self, x: u32, y: u32, channel: Channels) -> usize {
		channel * self.channel_stride + x as usize * self.x_stride + y as usize * self.y_stride
	}
}

impl<T> ChannelShrinkable<LooseLayout> for T
where
	T: Into<LooseLayout>,
{
	fn shrink_channels(self, channels: Channels) -> Result<LooseLayout> {
		let layout = self.into();
		ensure!(channels <= layout.channels, "Cannot shrink to more channels");
		Ok(LooseLayout { channels, ..layout })
	}
}

impl From<InterleavedLayout> for LooseLayout {
	fn from(value: InterleavedLayout) -> Self {
		Self {
			channels: value.channels(),
			width: value.width(),
			height: value.height(),
			channel_stride: 1,
			x_stride: value.x_stride(),
			y_stride: value.y_stride(),
		}
	}
}

impl From<PackedLayout> for LooseLayout {
	fn from(value: PackedLayout) -> Self {
		Into::<InterleavedLayout>::into(value).into()
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<const CHANNELS: Channels, C, B> ImageView<CHANNELS> for GenericImage<CHANNELS, C, LooseLayout, B>
where
	C: Color<CHANNELS>,
	B: Deref<Target = [C::Scalar]>,
{
	type Pixel = C;

	fn dimensions(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	fn get_pixel_unchecked(&self, x: u32, y: u32) -> Self::Pixel {
		let mut buffer = [Default::default(); CHANNELS];

		buffer.iter_mut().enumerate().for_each(|(channel, buf)| {
			let index = self.layout.component_index_unchecked(x, y, channel);
			let sample = (*self.buffer)[index];
			*buf = sample;
		});

		C::from_array(buffer)
	}
}

impl<const CHANNELS: Channels, C, B> ImageViewMut<CHANNELS> for GenericImage<CHANNELS, C, LooseLayout, B>
where
	C: Color<CHANNELS>,
	B: DerefMut<Target = [C::Scalar]>,
{
	fn put_pixel_unchecked(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
		// For each channel of the pixel, individually set the sample into the
		// underlying buffer
		for (channel, sample) in pixel.to_array().as_ref().iter().enumerate() {
			let index = self.layout.component_index_unchecked(x, y, channel);
			(*self.buffer)[index] = *sample;
		}
	}
}
