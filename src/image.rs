use std::{
	marker::PhantomData,
	ops::{Deref, DerefMut, Range},
};

use anyhow::{Context, Ok, Result, ensure};
use num_traits::FromBytes;

use crate::{
	AssumedLinear, AssumedSrgb, Channels, Color, ColorComponents, ConvertColorFrom, ImageIter, ImageIterMut,
	ImageLayout, ImageView, ImageViewMut, PixelView, PixelViewMut, spaces,
};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub mod interleaved;
pub mod loose;
pub mod packed;

pub use interleaved::*;
pub use loose::*;
pub use packed::*;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Image<C: Color, Layout: ImageLayout = PackedInterleavedLayout, Buffer = Vec<<C as Color>::Scalar>> {
	/// The backing buffer of the image, which contains the actual samples.
	pub(crate) buffer: Buffer,

	/// The layout of the image, which dictates the size of the image and how to locate pixels in the buffer.
	pub(crate) layout: Layout,

	/// Some PhatomData for our generic types
	_color: PhantomData<C>,
}

impl<C: Color, L: ImageLayout> Image<C, L, Vec<C::Scalar>> {
	/// Creates a new [`Image`] with a simple contiguous [`Vec`] as a buffer.
	pub fn new(width: u32, height: u32) -> Result<Self>
	where
		PackedInterleavedLayout: Into<L>,
	{
		let layout: L = PackedInterleavedLayout::new(width, height, InterleavedLayoutOrder::RowMajor)?.into();
		let buffer = vec![C::Scalar::default(); Self::CHANNELS * width as usize * height as usize];

		// Sanity check that the layout/buffer dimensions are correct
		debug_assert!(
			buffer.len() == layout.minimum_buffer_size(Self::CHANNELS),
			"The auto-generated buffer is not the exact size for the given layout, this is a bug."
		);

		Ok(Self {
			buffer,
			layout,
			_color: PhantomData,
		})
	}

	pub fn from_bytes<Bytes>(bytes: Bytes, layout: L) -> Result<Self>
	where
		Bytes: Deref<Target = [u8]>,
		C::Scalar: FromBytes<Bytes = [u8]>,
	{
		let scalar_buffer = bytes
			.chunks_exact(std::mem::size_of::<C::Scalar>() as usize)
			.map(TryInto::try_into)
			.map(Result::unwrap)
			.map(C::Scalar::from_ne_bytes)
			.collect::<Vec<C::Scalar>>();

		Self::from_buffer(scalar_buffer, layout)
	}
}

impl<C: Color, L: ImageLayout, B> Image<C, L, B> {
	pub const CHANNELS: Channels = C::CHANNELS;

	/// Creates a new [`Image`] instance given a backing buffer and an [`ImageLayout`].
	pub fn from_buffer(buffer: B, layout: L) -> Result<Self>
	where
		B: Deref<Target = [C::Scalar]>,
	{
		// Check that the layout/buffer dimensions are correct
		ensure!(
			buffer.len() >= layout.minimum_buffer_size(Self::CHANNELS),
			"The given buffer is too small to fit the entire image as dictated by the given layout."
		);

		Ok(Self {
			buffer,
			layout,
			_color: PhantomData,
		})
	}

	#[cfg(not(feature = "rayon"))]
	pub fn convert_color<C2>(mut self) -> Image<C2, L, B>
	where
		C: ColorComponents,
		C2: Color<Scalar = C::Scalar> + ColorComponents + ConvertColorFrom<C>,
		Self: ImageIterMut<Pixel = C>,
	{
		for pixel in self.iter_pixels_mut() {
			let color_in = pixel.as_color();
			let color_out = C2::color_from(color_in);
			for (dst, src) in pixel.slice.iter_mut().zip(color_out.to_array().as_ref()) {
				*dst = *src
			}
		}

		Image {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	/// TODO: Needs benchmarking to see if it's actually a good idea to use rayon here
	#[cfg(feature = "rayon")]
	pub fn convert_color<C2>(mut self) -> Image<C2, L, B>
	where
		C: ColorComponents,
		C2: Color<Scalar = C::Scalar> + ColorComponents + ConvertColorFrom<C>,
		Self: crate::ImageParallelIterMut<Pixel = C>,
	{
		use crate::ImageParallelIterMut;
		use rayon::iter::ParallelIterator;

		self.par_iter_pixels_mut().for_each(|pixel| {
			let color_in = pixel.as_color();
			let color_out = C2::color_from(color_in);
			for (dst, src) in pixel.slice.iter_mut().zip(color_out.to_array().as_ref()) {
				*dst = *src
			}
		});

		Image {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	pub fn layout(&self) -> L {
		self.layout
	}

	pub fn buffer(&self) -> &B {
		&self.buffer
	}

	pub fn into_buffer(self) -> B {
		self.buffer
	}

	pub fn assume_linear_rgb(self) -> Image<AssumedLinear<C>, L, B>
	where
		C: Color<Space = spaces::UnknownRGB>,
	{
		Image {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	pub fn assume_srgb(self) -> Image<AssumedSrgb<C>, L, B>
	where
		C: Color<Space = spaces::UnknownRGB>,
	{
		Image {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	fn pixel_range(&self, x: u32, y: u32) -> Range<usize> {
		let index = self.layout.index(x, y);
		let samples = Self::CHANNELS as usize;
		index..index + samples
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<C, B> ImageView for Image<C, B>
where
	C: Color + ColorComponents,
	B: Deref<Target = [C::Scalar]>,
{
	type Pixel = C;

	const CHANNELS: Channels = Self::CHANNELS;

	fn dimensions(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	fn get_pixel_unchecked(&self, x: u32, y: u32) -> Self::Pixel {
		// The channels are contiguous in the image so we can just access them as a slice
		let range = self.pixel_range(x, y);
		let pixel_slice = &self.buffer[range];

		C::from_slice_unchecked(pixel_slice)
	}
}

impl<C, B> ImageViewMut for Image<C, B>
where
	C: Color + ColorComponents,
	B: DerefMut<Target = [C::Scalar]>,
{
	fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) -> Result<()> {
		let range = self.pixel_range(x, y);
		let pixel_slice = self
			.buffer
			.get_mut(range)
			.context("Pixel x y coordinates out of bounds")?;

		pixel_slice.copy_from_slice(pixel.to_array().as_ref());

		Ok(())
	}

	fn put_pixel_unchecked(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
		let range = self.pixel_range(x, y);
		let pixel_slice = &mut self.buffer[range];

		pixel_slice.copy_from_slice(pixel.to_array().as_ref());
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<C, B> ImageIter for Image<C, B>
where
	C: Color,
	B: Deref<Target = [C::Scalar]>,
{
	type Pixel = C;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<C>> {
		// Layout is checked for well-formed-ness at image construction
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
		let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

		self.buffer.chunks_exact(major_stride).flat_map(move |padded_chunk| {
			let chunk = &padded_chunk[..minor_length as usize];

			chunk.chunks_exact(minor_stride as usize).map(move |padded_pixel| {
				let slice = &padded_pixel[..Self::CHANNELS];
				C::as_view_unchecked(slice)
			})
		})
	}

	/// Returns an iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<C>)> {
		// Layout is checked for well-formed-ness at image construction
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
		let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

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

impl<C, B> ImageIterMut for Image<C, B>
where
	C: Color,
	B: DerefMut<Target = [C::Scalar]>,
{
	type Pixel = C;

	/// Returns an iterator over the mutable pixels of the image, usable with `rayon`.
	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<C>> {
		// Layout is checked for well-formed-ness at image construction
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
		let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

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

	/// Returns an iterator over the mutable pixels of the image and their respective coordinates, usable with `rayon`.
	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<C>)> {
		// Layout is checked for well-formed-ness at image construction
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
		let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

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

	use crate::{ImageParallelIter, ImageParallelIterMut};

	use super::*;

	impl<C, B> ImageParallelIter for Image<C, B>
	where
		C: Color + Sync,
		C::Scalar: Sync,
		C::Format: Sync + Send,
		B: Deref<Target = [C::Scalar]> + Sync,
	{
		type Pixel = C;

		/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
		fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<C>> {
			// Layout is checked for well-formed-ness at image construction
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
			let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

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

		/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
		fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<C>)> {
			// Layout is checked for well-formed-ness at image construction
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
			let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

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

	#[cfg(feature = "rayon")]
	impl<C, B> ImageParallelIterMut for Image<C, B>
	where
		C: Color + Send + Sync,
		C::Scalar: Send + Sync,
		C::Format: Send + Sync,
		B: DerefMut<Target = [C::Scalar]> + Send + Sync,
	{
		type Pixel = C;

		/// Returns a parallel iterator over the mutable pixels of the image, usable with `rayon`.
		fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<C>> {
			// Layout is checked for well-formed-ness at image construction
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
			let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

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

		/// Returns a parallel iterator over the mutable pixels of the image and their respective coordinates, usable with `rayon`.
		fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<C>)> {
			// Layout is checked for well-formed-ness at image construction
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
			let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

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
