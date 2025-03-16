use std::ops::{Deref, DerefMut};

use anyhow::{Context, Result, ensure};

use crate::{
	Channels, Color, ColorComponents, ImageIter, ImageIterMut, ImageLayout, ImageView, ImageViewMut,
	InterleavedImageLayout, PixelView, PixelViewMut,
};

use super::{GenericImage, LooseLayout, PackedLayout};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the order in which the pixels are stored in memory.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InterleavedLayoutOrder {
	RowMajor,
	ColumnMajor,
}

impl InterleavedLayoutOrder {
	/// Compute the layout order from the strides and dimensions of the image.
	pub fn compute(x_stride: usize, y_stride: usize, width: u32, height: u32) -> Option<Self> {
		if y_stride >= x_stride * width as usize {
			Some(Self::RowMajor)
		} else if x_stride >= y_stride * height as usize {
			Some(Self::ColumnMajor)
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

/// Describes the layout of an interleaved image, where as opposed to planar, all channels are stored contiguously in memory.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InterleavedLayout {
	/// The width of the image.
	width: u32,

	/// The height of the image.
	height: u32,

	/// Add this stride to get to the next pixel in the x-direction.
	x_stride: usize,

	/// Add this stride to get to the next pixel in the y-direction.
	y_stride: usize,
}

impl InterleavedLayout {
	/// Create a new interleaved layout.
	pub fn new(width: u32, height: u32, x_stride: usize, y_stride: usize) -> Result<Self> {
		// Check that the layout is well-formed.
		ensure!(x_stride > 0, "X stride cannot be zero");
		ensure!(y_stride > 0, "Y stride cannot be zero");
		ensure!(
			InterleavedLayoutOrder::compute(x_stride, y_stride, width, height).is_some(),
			"Invalid strides"
		);

		Ok(Self {
			width,
			height,
			x_stride,
			y_stride,
		})
	}

	/// Get the x stride.
	pub fn x_stride(&self) -> usize {
		self.x_stride
	}

	/// Get the y stride.
	pub fn y_stride(&self) -> usize {
		self.y_stride
	}

	pub fn major_minor_strides(&self) -> (usize, usize) {
		if self.is_row_major() {
			(self.y_stride, self.x_stride)
		} else {
			(self.x_stride, self.y_stride)
		}
	}

	pub fn major_minor_sidelengths(&self) -> (u32, u32) {
		if self.is_row_major() {
			(self.width, self.height)
		} else {
			(self.height, self.width)
		}
	}
}

impl ImageLayout for InterleavedLayout {
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}

	fn minimum_buffer_size(&self, channels: Channels) -> usize {
		(self.x_stride * self.height as usize).max(self.y_stride * self.width as usize) * channels as usize
	}

	fn color_channel_index_unchecked(&self, channels: Channels, x: u32, y: u32, channel: Channels) -> usize {
		channel as usize + self.pixel_index_unchecked(channels, x, y)
	}
}

impl InterleavedImageLayout for InterleavedLayout {
	fn pixel_index_unchecked(&self, _channels: Channels, x: u32, y: u32) -> usize {
		x as usize * self.x_stride + y as usize * self.y_stride
	}

	fn order(&self) -> InterleavedLayoutOrder {
		InterleavedLayoutOrder::compute(self.x_stride, self.y_stride, self.width, self.height)
			.expect("Computed layout should be valid since it was checked in the constructor")
	}
}

impl From<PackedLayout> for InterleavedLayout {
	fn from(value: PackedLayout) -> Self {
		let order = value.order();

		match order {
			InterleavedLayoutOrder::RowMajor => Self {
				width: value.width(),
				height: value.height(),
				x_stride: 1,
				y_stride: value.width() as usize,
			},
			InterleavedLayoutOrder::ColumnMajor => Self {
				width: value.width(),
				height: value.height(),
				x_stride: value.height() as usize,
				y_stride: 1,
			},
		}
	}
}

impl TryFrom<LooseLayout> for InterleavedLayout {
	type Error = anyhow::Error;

	fn try_from(value: LooseLayout) -> Result<Self, Self::Error> {
		ensure!(value.channel_stride() == 1);

		Ok(Self {
			width: value.width(),
			height: value.height(),
			x_stride: value.x_stride(),
			y_stride: value.y_stride(),
		})
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<C, L, B> ImageView for GenericImage<C, L, B>
where
	C: Color + ColorComponents,
	L: InterleavedImageLayout,
	B: Deref<Target = [C::Scalar]>,
{
	type Pixel = C;

	const CHANNELS: Channels = Self::CHANNELS;

	fn dimensions(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	fn get_pixel_unchecked(&self, x: u32, y: u32) -> Self::Pixel {
		// The channels are interleaved in the image so we can just access them as a slice
		let range = self.layout.pixel_range_unchecked(Self::CHANNELS, x, y);
		let pixel_slice = &self.buffer[range];

		C::from_slice_unchecked(pixel_slice)
	}
}

impl<C, L, B> ImageViewMut for GenericImage<C, L, B>
where
	C: Color + ColorComponents,
	L: InterleavedImageLayout,
	B: DerefMut<Target = [C::Scalar]>,
{
	fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) -> Result<()> {
		let range = self.layout.pixel_range_unchecked(Self::CHANNELS, x, y);
		let pixel_slice = self
			.buffer
			.get_mut(range)
			.context("Pixel x y coordinates out of bounds")?;

		pixel_slice.copy_from_slice(pixel.to_array().as_ref());

		Ok(())
	}

	fn put_pixel_unchecked(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
		let range = self.layout.pixel_range_unchecked(Self::CHANNELS, x, y);
		let pixel_slice = &mut self.buffer[range];

		pixel_slice.copy_from_slice(pixel.to_array().as_ref());
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<C, B> ImageIter for GenericImage<C, InterleavedLayout, B>
where
	C: Color,
	B: Deref<Target = [C::Scalar]>,
{
	type Pixel = C;

	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<C>> {
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides();
		let (_, minor_length) = layout.major_minor_sidelengths();

		self.buffer.chunks_exact(major_stride).flat_map(move |padded_chunk| {
			let chunk = &padded_chunk[..minor_length as usize];

			chunk.chunks_exact(minor_stride as usize).map(move |padded_pixel| {
				let slice = &padded_pixel[..Self::CHANNELS];
				C::as_view_unchecked(slice)
			})
		})
	}

	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<C>)> {
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides();
		let (_, minor_length) = layout.major_minor_sidelengths();

		self.buffer
			.chunks_exact(major_stride)
			.enumerate()
			.flat_map(move |(major_index, padded_chunk)| {
				let chunk = &padded_chunk[..minor_length as usize];

				chunk
					.chunks_exact(minor_stride)
					.enumerate()
					.map(move |(minor_index, padded_pixel)| {
						let (x, y) = if layout.is_row_major() {
							(minor_index, major_index)
						} else {
							(major_index, minor_index)
						};
						let slice = &padded_pixel[..Self::CHANNELS];
						let pixel = C::as_view_unchecked(slice);

						(x as u32, y as u32, pixel)
					})
			})
	}
}

impl<C, B> ImageIterMut for GenericImage<C, InterleavedLayout, B>
where
	C: Color,
	B: DerefMut<Target = [C::Scalar]>,
{
	type Pixel = C;

	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<C>> {
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides();
		let (_, minor_length) = layout.major_minor_sidelengths();

		self.buffer
			.chunks_exact_mut(major_stride)
			.flat_map(move |padded_chunk| {
				let chunk = &mut padded_chunk[..minor_length as usize];

				chunk.chunks_exact_mut(minor_stride).map(move |padded_pixel| {
					let slice = &mut padded_pixel[..Self::CHANNELS];
					C::as_view_mut_unchecked(slice)
				})
			})
	}

	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<C>)> {
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides();
		let (_, minor_length) = layout.major_minor_sidelengths();

		self.buffer
			.chunks_exact_mut(major_stride)
			.enumerate()
			.flat_map(move |(major_index, padded_chunk)| {
				let chunk = &mut padded_chunk[..minor_length as usize];

				chunk
					.chunks_exact_mut(minor_stride)
					.enumerate()
					.map(move |(minor_index, padded_pixel)| {
						let (x, y) = if layout.is_row_major() {
							(minor_index, major_index)
						} else {
							(major_index, minor_index)
						};
						let slice = &mut padded_pixel[..Self::CHANNELS];
						let pixel = C::as_view_mut_unchecked(slice);

						(x as u32, y as u32, pixel)
					})
			})
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[cfg(feature = "rayon")]
mod par_iter {
	use rayon::{
		iter::{IndexedParallelIterator, ParallelIterator},
		slice::{ParallelSlice, ParallelSliceMut},
	};

	use crate::{ImageParallelIter, ImageParallelIterMut, PixelViewMut};

	use super::*;

	impl<C, B> ImageParallelIter for GenericImage<C, InterleavedLayout, B>
	where
		C: Color + Sync,
		C::Scalar: Sync,
		C::Format: Sync + Send,
		B: Deref<Target = [C::Scalar]> + Sync,
	{
		type Pixel = C;

		fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<C>> {
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides();
			let (_, minor_length) = layout.major_minor_sidelengths();

			self.buffer
				.par_chunks_exact(major_stride)
				.flat_map(move |padded_chunk| {
					let chunk = &padded_chunk[..minor_length as usize];

					chunk.par_chunks_exact(minor_stride).map(move |padded_pixel| {
						let slice = &padded_pixel[..Self::CHANNELS];
						C::as_view_unchecked(slice)
					})
				})
		}

		fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<C>)> {
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides();
			let (_, minor_length) = layout.major_minor_sidelengths();

			self.buffer
				.par_chunks_exact(major_stride)
				.enumerate()
				.flat_map(move |(major_index, padded_chunk)| {
					let chunk = &padded_chunk[..minor_length as usize];

					chunk
						.par_chunks_exact(minor_stride)
						.enumerate()
						.map(move |(minor_index, padded_pixel)| {
							let (x, y) = if layout.is_row_major() {
								(minor_index, major_index)
							} else {
								(major_index, minor_index)
							};
							let slice = &padded_pixel[..Self::CHANNELS];
							let pixel = C::as_view_unchecked(slice);

							(x as u32, y as u32, pixel)
						})
				})
		}
	}

	impl<C, B> ImageParallelIterMut for GenericImage<C, InterleavedLayout, B>
	where
		C: Color + Send + Sync,
		C::Scalar: Send + Sync,
		C::Format: Send + Sync,
		B: DerefMut<Target = [C::Scalar]> + Send + Sync,
	{
		type Pixel = C;

		fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<C>> {
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides();
			let (_, minor_length) = layout.major_minor_sidelengths();

			self.buffer
				.par_chunks_exact_mut(major_stride)
				.flat_map(move |padded_chunk| {
					let chunk = &mut padded_chunk[..minor_length as usize];

					chunk.par_chunks_exact_mut(minor_stride).map(move |padded_pixel| {
						let slice = &mut padded_pixel[..Self::CHANNELS];
						C::as_view_mut_unchecked(slice)
					})
				})
		}

		fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<C>)> {
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides();
			let (_, minor_length) = layout.major_minor_sidelengths();

			self.buffer
				.par_chunks_exact_mut(major_stride)
				.enumerate()
				.flat_map(move |(major_index, padded_chunk)| {
					let chunk = &mut padded_chunk[..minor_length as usize];

					chunk
						.par_chunks_exact_mut(minor_stride)
						.enumerate()
						.map(move |(minor_index, padded_pixel)| {
							let (x, y) = if layout.is_row_major() {
								(minor_index, major_index)
							} else {
								(major_index, minor_index)
							};
							let slice = &mut padded_pixel[..Self::CHANNELS];
							let pixel = C::as_view_mut_unchecked(slice);

							(x as u32, y as u32, pixel)
						})
				})
		}
	}
}
