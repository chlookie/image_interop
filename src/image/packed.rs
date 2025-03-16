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
	fn width(&self) -> u32 {
		self.width
	}

	fn height(&self) -> u32 {
		self.height
	}

	fn minimum_buffer_size(&self, channels: Channels) -> usize {
		self.width as usize * self.height as usize * channels as usize
	}

	fn color_channel_index_unchecked(&self, channels: Channels, x: u32, y: u32, channel: Channels) -> usize {
		channel as usize + self.pixel_index_unchecked(channels, x, y)
	}
}

impl InterleavedImageLayout for PackedLayout {
	fn pixel_index_unchecked(&self, channels: Channels, x: u32, y: u32) -> usize {
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

impl<C, B> ImageIter for GenericImage<C, PackedLayout, B>
where
	C: Color,
	B: Deref<Target = [C::Scalar]>,
{
	type Pixel = C;

	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<C>> {
		self.buffer.chunks_exact(Self::CHANNELS).map(C::as_view_unchecked)
	}

	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<C>)> {
		self.buffer
			.chunks_exact(Self::CHANNELS)
			.enumerate()
			.map(|(index, slice)| {
				let (x, y) = self.layout.reverse_index(index);
				(x, y, C::as_view_unchecked(slice))
			})
	}
}

impl<C, B> ImageIterMut for GenericImage<C, PackedLayout, B>
where
	C: Color,
	B: DerefMut<Target = [C::Scalar]>,
{
	type Pixel = C;

	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<C>> {
		self.buffer
			.chunks_exact_mut(Self::CHANNELS)
			.map(C::as_view_mut_unchecked)
	}

	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<C>)> {
		self.buffer
			.chunks_exact_mut(Self::CHANNELS)
			.enumerate()
			.map(|(index, slice)| {
				let (x, y) = self.layout.reverse_index(index);
				(x, y, C::as_view_mut_unchecked(slice))
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

	impl<C, B> ImageParallelIter for GenericImage<C, PackedLayout, B>
	where
		C: Color + Sync,
		C::Scalar: Sync,
		C::Format: Sync + Send,
		B: Deref<Target = [C::Scalar]> + Sync,
	{
		type Pixel = C;

		fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<C>> {
			self.buffer.par_chunks_exact(Self::CHANNELS).map(C::as_view_unchecked)
		}

		fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<C>)> {
			self.buffer
				.par_chunks_exact(Self::CHANNELS)
				.enumerate()
				.map(|(index, slice)| {
					let (x, y) = self.layout.reverse_index(index);
					(x, y, C::as_view_unchecked(slice))
				})
		}
	}

	impl<C, B> ImageParallelIterMut for GenericImage<C, PackedLayout, B>
	where
		C: Color + Send + Sync,
		C::Scalar: Send + Sync,
		C::Format: Send + Sync,
		B: DerefMut<Target = [C::Scalar]> + Send + Sync,
	{
		type Pixel = C;

		fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<C>> {
			self.buffer
				.par_chunks_exact_mut(Self::CHANNELS)
				.map(C::as_view_mut_unchecked)
		}

		fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<C>)> {
			self.buffer
				.par_chunks_exact_mut(Self::CHANNELS)
				.enumerate()
				.map(|(index, slice)| {
					let (x, y) = self.layout.reverse_index(index);
					(x, y, C::as_view_mut_unchecked(slice))
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
		assert_eq!(layout.color_channel_index_unchecked(3, 5, 12, 0), 5 + 12 * 10 * 3);

		let layout = PackedLayout::new(90, 5, InterleavedLayoutOrder::ColumnMajor).unwrap();
		assert_eq!(layout.color_channel_index_unchecked(4, 7, 10, 1), 10 + 7 * 5 * 4 + 1);
	}
}
