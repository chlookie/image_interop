use std::{
	marker::PhantomData,
	ops::{Deref, DerefMut, Range},
};

use anyhow::{Context, Ok, Result, ensure};
use num_traits::FromBytes;

use crate::{
	Channels, ImageIter, ImageIterMut, ImageLayout, ImageView, ImageViewMut, Pixel, PixelToComponents, PixelView,
	PixelViewMut,
};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// A generic image container that can store pixels of any type that implements the [`Pixel`] trait.
/// The image data can be stored in any buffer type that implements the necessary traits (typically [`Vec`]).
///
/// The image supports both row-major and column-major layouts, with configurable strides for both
/// dimensions. This allows for efficient representation of sub-images and different memory layouts.
///
/// # Type Parameters
///
/// * `P`: The pixel type, which must implement the [`Pixel`] trait
/// * `Buffer`: The underlying buffer type, defaults to `Vec<<P as Pixel>::Scalar>`
///
/// # Examples
///
/// ```
/// use image_interop::{Image, Rgb};
///
/// // Create a new 800x600 RGB image
/// let mut image: Image<Rgb> = Image::new(800, 600);
///
/// // Access pixels
/// image.put_pixel(0, 0, Rgb::new(255, 0, 0)); // Set first pixel to red
///
/// // Iterate over all pixels
/// for (x, y, pixel) in image.enumerate_pixels() {
///     // Process each pixel
/// }
/// ```
///
/// The image can also be created from existing data:
///
/// ```
/// use image_interop::{Image, ImageLayout, Rgb};
///
/// let buffer = vec![0u8; 800 * 600 * 3];
/// let layout = ImageLayout::row_major_packed(3, 800, 600);
/// let image = Image::<Rgb, _>::from_buffer(buffer, layout).unwrap();
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Image<P: Pixel, Buffer = Vec<<P as Pixel>::Scalar>> {
	/// The backing buffer of the image, which contains the actual samples.
	pub(crate) buffer: Buffer,

	/// The layout of the image, which dictates the size of the image and how to locate pixels in the buffer.
	pub(crate) layout: ImageLayout,

	/// Some PhatomData for our generic types
	_pixel: PhantomData<P>,
}

impl<P: Pixel> Image<P, Vec<P::Scalar>> {
	/// Creates a new [`Image`] with a simple contiguous [`Vec`] as a buffer.
	pub fn new(width: u32, height: u32) -> Self {
		let layout = ImageLayout::row_major_packed(Self::CHANNELS, width, height);
		let buffer = vec![P::Scalar::default(); Self::CHANNELS * width as usize * height as usize];

		// Sanity check that the layout is well-formed
		debug_assert!(
			layout.is_well_formed(),
			"The auto-generated layout is malformed, this is a bug."
		);

		// Sanity check that the layout/buffer dimensions are correct
		debug_assert!(
			buffer.len() == Self::expected_total_samples(layout),
			"The auto-generated buffer is not the exact size for the given layout, this is a bug."
		);

		Self {
			buffer,
			layout,
			_pixel: PhantomData,
		}
	}

	pub fn from_bytes<B>(bytes: B, layout: ImageLayout) -> Result<Self>
	where
		B: Deref<Target = [u8]>,
		P::Scalar: FromBytes<Bytes = [u8]>,
	{
		let scalar_buffer = bytes
			.chunks_exact(std::mem::size_of::<P::Scalar>() as usize)
			.map(TryInto::try_into)
			.map(Result::unwrap)
			.map(P::Scalar::from_ne_bytes)
			.collect::<Vec<P::Scalar>>();

		Self::from_buffer(scalar_buffer, layout)
	}
}

impl<Buffer, P: Pixel> Image<P, Buffer> {
	pub const CHANNELS: Channels = P::CHANNELS;

	/// Creates a new [`Image`] instance given a backing buffer and an [`ImageLayout`].
	pub fn from_buffer(buffer: Buffer, layout: ImageLayout) -> Result<Self>
	where
		Buffer: Deref<Target = [P::Scalar]>,
	{
		// Check that the layout is well-formed
		ensure!(layout.is_well_formed(), "The given layout is malformed.");

		// Check that the layout/buffer dimensions are correct
		ensure!(
			buffer.len() >= Self::expected_total_samples(layout),
			"The given buffer is too small to fit the entire image as dictated by the given layout."
		);

		Ok(Self {
			buffer,
			layout,
			_pixel: PhantomData,
		})
	}

	pub fn layout(&self) -> ImageLayout {
		self.layout
	}

	pub fn buffer(&self) -> &Buffer {
		&self.buffer
	}

	pub fn into_buffer(self) -> Buffer {
		self.buffer
	}

	pub fn total_samples(&self) -> usize {
		Self::expected_total_samples(self.layout)
	}

	pub fn expected_total_samples(layout: ImageLayout) -> usize {
		// Layout is checked for well-formed-ness at image construction
		layout.total_padded_pixels().unwrap() * Self::CHANNELS
	}

	fn pixel_range(&self, x: u32, y: u32) -> Range<usize> {
		let index = self.layout.index(x, y);
		let samples = Self::CHANNELS as usize;
		index..index + samples
	}
}

impl<Buffer, P> Image<P, Buffer>
where
	P: Pixel,
	Self: ImageIter<Pixel = P> + ImageView,
{
	pub fn to_packed(self) -> Image<P, Vec<<P as Pixel>::Scalar>>
	where
		P: PixelToComponents,
	{
		let mut image_out: Image<P, Vec<<P as Pixel>::Scalar>> = Image::new(self.width(), self.height());

		for (x, y, from) in self.enumerate_pixels() {
			image_out.put_pixel_unchecked(x, y, from.as_pixel());
		}

		image_out
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<Buffer, P> ImageView for Image<P, Buffer>
where
	P: Pixel + PixelToComponents,
	Buffer: Deref<Target = [P::Scalar]>,
{
	type Pixel = P;

	fn dimensions(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	fn get_pixel_unchecked(&self, x: u32, y: u32) -> Self::Pixel {
		// The channels are contiguous in the image so we can just access them as a slice
		let range = self.pixel_range(x, y);
		let pixel_slice = &self.buffer[range];

		P::from_slice_unchecked(pixel_slice)
	}
}

impl<Buffer, P> ImageViewMut for Image<P, Buffer>
where
	P: Pixel + PixelToComponents,
	Buffer: DerefMut<Target = [P::Scalar]>,
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

impl<P, Buffer> ImageIter for Image<P, Buffer>
where
	P: Pixel,
	Buffer: Deref<Target = [P::Scalar]>,
{
	type Pixel = P;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<P>> {
		// Layout is checked for well-formed-ness at image construction
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
		let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

		self.buffer.chunks_exact(major_stride).flat_map(move |padded_chunk| {
			let chunk = &padded_chunk[..minor_length as usize];

			chunk.chunks_exact(minor_stride as usize).map(move |padded_pixel| {
				let slice = &padded_pixel[..Self::CHANNELS];
				P::as_view_unchecked(slice)
			})
		})
	}

	/// Returns an iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<P>)> {
		// Layout is checked for well-formed-ness at image construction
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
		let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

		self.buffer
			.chunks_exact(major_stride)
			.enumerate()
			.flat_map(move |(outer_index, padded_chunk)| {
				let chunk = &padded_chunk[..minor_length as usize];

				chunk
					.chunks_exact(minor_stride)
					.enumerate()
					.map(move |(inner_index, padded_pixel)| {
						let (x, y) = if layout.is_row_major() {
							(inner_index, outer_index)
						} else {
							(outer_index, inner_index)
						};
						let slice = &padded_pixel[..Self::CHANNELS];
						let pixel = P::as_view_unchecked(slice);

						(x as u32, y as u32, pixel)
					})
			})
	}
}

impl<P, Buffer> ImageIterMut for Image<P, Buffer>
where
	P: Pixel,
	Buffer: DerefMut<Target = [P::Scalar]>,
{
	type Pixel = P;

	/// Returns an iterator over the mutable pixels of the image, usable with `rayon`.
	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<P>> {
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
					P::as_view_mut_unchecked(slice)
				})
			})
	}

	/// Returns an iterator over the mutable pixels of the image and their respective coordinates, usable with `rayon`.
	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<P>)> {
		// Layout is checked for well-formed-ness at image construction
		let layout = self.layout;
		let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
		let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

		self.buffer
			.chunks_exact_mut(major_stride)
			.enumerate()
			.flat_map(move |(outer_index, padded_chunk)| {
				let chunk = &mut padded_chunk[..minor_length as usize];

				chunk
					.chunks_exact_mut(minor_stride)
					.enumerate()
					.map(move |(inner_index, padded_pixel)| {
						let (x, y) = if layout.is_row_major() {
							(inner_index, outer_index)
						} else {
							(outer_index, inner_index)
						};
						let slice = &mut padded_pixel[..Self::CHANNELS];
						let pixel = P::as_view_mut_unchecked(slice);

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

	impl<P, Buffer> ImageParallelIter for Image<P, Buffer>
	where
		P: Pixel + Sync,
		P::Scalar: Sync,
		P::Format: Sync + Send,
		Buffer: Deref<Target = [P::Scalar]> + Sync,
	{
		type Pixel = P;

		/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
		fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<P>> {
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
						P::as_view_unchecked(slice)
					})
				})
		}

		/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
		fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<P>)> {
			// Layout is checked for well-formed-ness at image construction
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
			let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

			self.buffer
				.par_chunks_exact(major_stride)
				.enumerate()
				.flat_map(move |(outer_index, padded_chunk)| {
					let chunk = &padded_chunk[..minor_length as usize];

					chunk
						.par_chunks_exact(minor_stride)
						.enumerate()
						.map(move |(inner_index, padded_pixel)| {
							let (x, y) = if layout.is_row_major() {
								(inner_index, outer_index)
							} else {
								(outer_index, inner_index)
							};
							let slice = &padded_pixel[..Self::CHANNELS];
							let pixel = P::as_view_unchecked(slice);

							(x as u32, y as u32, pixel)
						})
				})
		}
	}

	#[cfg(feature = "rayon")]
	impl<P, Buffer> ImageParallelIterMut for Image<P, Buffer>
	where
		P: Pixel + Send + Sync,
		P::Scalar: Send + Sync,
		P::Format: Send + Sync,
		Buffer: DerefMut<Target = [P::Scalar]> + Send + Sync,
	{
		type Pixel = P;

		/// Returns a parallel iterator over the mutable pixels of the image, usable with `rayon`.
		fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<P>> {
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
						P::as_view_mut_unchecked(slice)
					})
				})
		}

		/// Returns a parallel iterator over the mutable pixels of the image and their respective coordinates, usable with `rayon`.
		fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<P>)> {
			// Layout is checked for well-formed-ness at image construction
			let layout = self.layout;
			let (major_stride, minor_stride) = layout.major_minor_strides().unwrap();
			let (_, minor_length) = layout.major_minor_sidelengths().unwrap();

			self.buffer
				.par_chunks_exact_mut(major_stride)
				.enumerate()
				.flat_map(move |(outer_index, padded_chunk)| {
					let chunk = &mut padded_chunk[..minor_length as usize];

					chunk
						.par_chunks_exact_mut(minor_stride)
						.enumerate()
						.map(move |(inner_index, padded_pixel)| {
							let (x, y) = if layout.is_row_major() {
								(inner_index, outer_index)
							} else {
								(outer_index, inner_index)
							};
							let slice = &mut padded_pixel[..Self::CHANNELS];
							let pixel = P::as_view_mut_unchecked(slice);

							(x as u32, y as u32, pixel)
						})
				})
		}
	}
}
