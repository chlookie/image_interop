use std::ops::{Deref, DerefMut};

use anyhow::{Result, ensure};

use crate::{Channels, Color, ImageIter, ImageIterMut, ImageLayout, InterleavedImageLayout, PixelView, PixelViewMut};

use super::{GenericImage, InterleavedLayout, InterleavedLayoutOrder};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes the layout of a packed image, where each row/column of pixels is stored contiguously in memory, with no padding in between.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PackedLayout {
	/// The number of color channels in the image.
	channels: Channels,

	/// The width of the image.
	width: u32,

	/// The height of the image.
	height: u32,

	/// The storage order of the image, either column major or row major.
	order: InterleavedLayoutOrder,
}

impl PackedLayout {
	/// Create a new packed layout.
	pub fn new(channels: Channels, width: u32, height: u32, order: InterleavedLayoutOrder) -> Result<Self> {
		ensure!(channels > 0, "Channels cannot be zero");
		ensure!(width > 0, "Width cannot be zero");
		ensure!(height > 0, "Height cannot be zero");

		Ok(Self {
			channels,
			width,
			height,
			order,
		})
	}

	/// Get the major and minor sidelengths of the image, i.e. the width and height if row major, or the height and width if column major.
	pub fn major_minor_sidelengths(&self) -> (u32, u32) {
		if self.is_row_major() {
			(self.width, self.height)
		} else {
			(self.height, self.width)
		}
	}

	/// Get the coordinates of the pixel at the given index.
	fn reverse_index(&self, index: usize) -> (u32, u32) {
		if self.is_row_major() {
			(
				(index % self.width as usize) as u32,
				(index / self.width as usize) as u32,
			)
		} else {
			(
				(index / self.height as usize) as u32,
				(index % self.height as usize) as u32,
			)
		}
	}
}

impl ImageLayout for PackedLayout {
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
		self.width as usize * self.height as usize * self.channels as usize
	}

	fn component_index_unchecked(&self, x: u32, y: u32, channel: Channels) -> usize {
		channel as usize + self.pixel_index_unchecked(x, y)
	}
}

impl InterleavedImageLayout for PackedLayout {
	fn pixel_index_unchecked(&self, x: u32, y: u32) -> usize {
		match self.order {
			InterleavedLayoutOrder::RowMajor => x as usize + y as usize * self.width as usize * self.channels,
			InterleavedLayoutOrder::ColumnMajor => y as usize + x as usize * self.height as usize * self.channels,
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
			channels: value.channels(),
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

impl<const CHANNELS: Channels, C, B> ImageIter<CHANNELS> for GenericImage<CHANNELS, C, PackedLayout, B>
where
	C: Color<CHANNELS>,
	B: Deref<Target = [C::Scalar]>,
{
	type Pixel = C;

	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<'_, CHANNELS, C>> {
		self.buffer
			.chunks_exact(CHANNELS)
			.map(TryInto::try_into)
			.map(Result::unwrap)
			.map(C::as_view)
	}

	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<'_, CHANNELS, C>)> {
		self.buffer
			.chunks_exact(CHANNELS)
			.map(TryInto::try_into)
			.map(Result::unwrap)
			.enumerate()
			.map(|(index, array)| {
				let (x, y) = self.layout.reverse_index(index);
				(x, y, C::as_view(array))
			})
	}
}

impl<const CHANNELS: Channels, C, B> ImageIterMut<CHANNELS> for GenericImage<CHANNELS, C, PackedLayout, B>
where
	C: Color<CHANNELS>,
	B: DerefMut<Target = [C::Scalar]>,
{
	type Pixel = C;

	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<'_, CHANNELS, C>> {
		self.buffer
			.chunks_exact_mut(CHANNELS)
			.map(TryInto::try_into)
			.map(Result::unwrap)
			.map(C::as_view_mut)
	}

	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<'_, CHANNELS, C>)> {
		self.buffer
			.chunks_exact_mut(CHANNELS)
			.map(TryInto::try_into)
			.map(Result::unwrap)
			.enumerate()
			.map(|(index, array)| {
				let (x, y) = self.layout.reverse_index(index);
				(x, y, C::as_view_mut(array))
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
	use std::ops::Deref;

	use rayon::{
		iter::{IndexedParallelIterator, ParallelIterator},
		slice::{ParallelSlice, ParallelSliceMut},
	};

	use crate::{ImageParallelIter, ImageParallelIterMut, PixelViewMut};

	use super::*;

	impl<const CHANNELS: Channels, C, B> ImageParallelIter<CHANNELS> for GenericImage<CHANNELS, C, PackedLayout, B>
	where
		C: Color<CHANNELS> + Sync,
		C::Scalar: Sync,
		C::Format: Sync + Send,
		B: Deref<Target = [C::Scalar]> + Sync,
	{
		type Pixel = C;

		fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<'_, CHANNELS, C>> {
			self.buffer
				.par_chunks_exact(CHANNELS)
				.map(TryInto::try_into)
				.map(Result::unwrap)
				.map(C::as_view)
		}

		fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<'_, CHANNELS, C>)> {
			self.buffer
				.par_chunks_exact(CHANNELS)
				.map(TryInto::try_into)
				.map(Result::unwrap)
				.enumerate()
				.map(|(index, array)| {
					let (x, y) = self.layout.reverse_index(index);
					(x, y, C::as_view(array))
				})
		}
	}

	impl<const CHANNELS: Channels, C, B> ImageParallelIterMut<CHANNELS> for GenericImage<CHANNELS, C, PackedLayout, B>
	where
		C: Color<CHANNELS> + Send + Sync,
		C::Scalar: Send + Sync,
		C::Format: Send + Sync,
		B: DerefMut<Target = [C::Scalar]> + Send + Sync,
	{
		type Pixel = C;

		fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<'_, CHANNELS, C>> {
			self.buffer
				.par_chunks_exact_mut(CHANNELS)
				.map(TryInto::try_into)
				.map(Result::unwrap)
				.map(C::as_view_mut)
		}

		fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<'_, CHANNELS, C>)> {
			self.buffer
				.par_chunks_exact_mut(CHANNELS)
				.map(TryInto::try_into)
				.map(Result::unwrap)
				.enumerate()
				.map(|(index, array)| {
					let (x, y) = self.layout.reverse_index(index);
					(x, y, C::as_view_mut(array))
				})
		}
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[cfg(test)]
mod tests {}
