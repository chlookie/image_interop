use std::{
	marker::PhantomData,
	ops::{Deref, DerefMut, Range},
};

use anyhow::{Context, Ok, Result, ensure};
use num_traits::FromBytes;

use crate::{Channels, ImageIter, Pixel, PixelToComponents};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ImageLayout {
	/// The width of the image.
	pub width: u32,

	/// The height of the image.
	pub height: u32,

	/// Add this stride to get to the next pixel in the x-direction.
	pub stride_x: u32,

	/// Add this stride to get to the next pixel in the y-direction.
	pub stride_y: u32,
}

impl ImageLayout {
	pub fn row_major_packed(channels: Channels, width: u32, height: u32) -> Self {
		ImageLayout {
			width,
			height,
			stride_x: channels as u32,
			stride_y: channels as u32 * width,
		}
	}

	pub fn column_major_packed(channels: Channels, width: u32, height: u32) -> Self {
		ImageLayout {
			width,
			height,
			stride_x: channels as u32 * height,
			stride_y: channels as u32,
		}
	}

	pub fn is_row_major(&self) -> bool {
		self.stride_x < self.stride_y
	}

	pub fn is_column_major(&self) -> bool {
		self.stride_x > self.stride_y
	}

	fn total_pixels(&self) -> Option<u32> {
		// Since we are using strides, can't just do width*height
		if self.is_row_major() {
			// stride_y > stride_x
			self.stride_y.checked_mul(self.height)
		} else if self.is_column_major() {
			// stride_x > stride_y
			self.stride_x.checked_mul(self.width)
		} else {
			// Layout is malformed
			None
		}
	}

	pub fn index(&self, x: u32, y: u32) -> usize {
		(x * self.stride_x + y * self.stride_y) as usize
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// A custom image type.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Image<P: Pixel, Buffer = Vec<<P as Pixel>::Scalar>, Sample = [<P as Pixel>::Scalar; 1]> {
	/// The backing buffer of the image, which contains the actual samples.
	pub(crate) buffer: Buffer,

	/// The layout of the image, which dictates the size of the image and how to locate pixels in the buffer.
	pub(crate) layout: ImageLayout,

	/// Some PhatomData for our generic types
	_pixel: PhantomData<P>,
	_sample: PhantomData<Sample>,
}

impl<P: Pixel> Image<P, Vec<P::Scalar>> {
	/// Creates a new [`Image`] with a simple contiguous [`Vec`] as a buffer.
	pub fn new(width: u32, height: u32) -> Self {
		let buffer = vec![P::Scalar::default(); Self::CHANNELS * width as usize * height as usize];

		let layout = ImageLayout::row_major_packed(Self::CHANNELS, width, height);

		// Sanity check that the layout/buffer dimensions are correct
		debug_assert!(
			buffer.len() == Self::expected_buffer_size(layout),
			"The auto-generated buffer is not the exact size for the given layout, this is a bug."
		);

		Self {
			buffer,
			layout,
			_pixel: PhantomData,
			_sample: PhantomData,
		}
	}
}

impl<Buffer, P: Pixel, const SAMPLE_LEN: usize, S> Image<P, Buffer, [S; SAMPLE_LEN]>
where
	Buffer: Deref<Target = [S]>,
{
	/// Creates a new [`Image`] instance given a backing buffer and an [`ImageLayout`].
	pub fn from_buffer(buffer: Buffer, layout: ImageLayout) -> Result<Self> {
		// Sanity check that the layout/buffer dimensions are correct
		ensure!(
			buffer.len() >= Self::expected_buffer_size(layout),
			"The given buffer is too small to fit the entire image as dictated by the given layout."
		);

		Ok(Self {
			buffer,
			layout,
			_pixel: PhantomData,
			_sample: PhantomData,
		})
	}
}

impl<Buffer, P: Pixel, const SAMPLE_LEN: usize, S> Image<P, Buffer, [S; SAMPLE_LEN]> {
	pub const CHANNELS: Channels = P::CHANNELS;

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
		let pixels = layout.total_pixels().unwrap();
		pixels as usize * Self::CHANNELS
	}

	pub fn expected_buffer_size(layout: ImageLayout) -> usize {
		Self::expected_total_samples(layout) * SAMPLE_LEN
	}

	fn pixel_range(&self, x: u32, y: u32) -> Range<usize> {
		let index = self.layout.index(x, y);
		let samples = Self::CHANNELS as usize * SAMPLE_LEN;
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

/// Trait to inspect an image.
pub trait ImageView {
	/// The type of pixel.
	type Pixel: Pixel;

	/// Returns the number of channels the color format of the image has.
	fn channels(&self) -> Channels {
		Self::Pixel::CHANNELS
	}

	/// The width and height of this image.
	fn dimensions(&self) -> (u32, u32);

	/// The width of this image.
	fn width(&self) -> u32 {
		self.dimensions().0
	}

	/// The height of this image.
	fn height(&self) -> u32 {
		self.dimensions().1
	}

	/// Returns true if this x, y coordinate is contained inside the image.
	fn in_bounds(&self, x: u32, y: u32) -> bool {
		let (width, height) = self.dimensions();
		x < width && y < height
	}

	/// Returns the pixel located at (x, y). Indexed from top left.
	fn get_pixel(&self, x: u32, y: u32) -> Result<Self::Pixel> {
		ensure!(
			x < self.width() && y < self.height(),
			"Pixel x y coordinates out of bounds"
		);

		Ok(self.get_pixel_unchecked(x, y))
	}

	/// Returns the pixel located at (x, y). Indexed from top left.
	///
	/// # Panics
	///
	/// Panics if `(x, y)` is out of bounds.
	fn get_pixel_unchecked(&self, x: u32, y: u32) -> Self::Pixel;
}

/// A trait for manipulating images.
pub trait ImageViewMut: ImageView {
	/// Put a pixel at location (x, y). Indexed from top left.
	fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) -> Result<()>;

	/// Put a pixel at location (x, y). Indexed from top left.
	///
	/// # Panics
	///
	/// Panics if `(x, y)` is out of bounds.
	fn put_pixel_unchecked(&mut self, x: u32, y: u32, pixel: Self::Pixel);

	/// Copies all of the pixels from another image into this image.
	///
	/// Both images have to have matching sizes.
	fn copy_from<O>(&mut self, other: &O, x: u32, y: u32) -> Result<()>
	where
		O: ImageView<Pixel = Self::Pixel>,
	{
		// Do bounds checking here so we can use the non-bounds-checking functions to copy pixels.
		ensure!(
			self.width() == other.width() && self.height() == other.height(),
			"Image sizes do not match"
		);

		for k in 0..self.height() {
			for i in 0..self.width() {
				let p = other.get_pixel_unchecked(i, k);
				self.put_pixel_unchecked(i + x, k + y, p);
			}
		}
		Ok(())
	}
}

impl<Buffer, P> ImageView for Image<P, Buffer, [P::Scalar; 1]>
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

// macro_rules! impl_image_view_for_byte_buffers {
// 	($bytes:expr) => {
impl<Buffer, P> ImageView for Image<P, Buffer, [u8; 2]>
where
	P: Pixel + PixelToComponents,
	Buffer: Deref<Target = [u8]>,
	P::Scalar: FromBytes<Bytes = [u8]>,
{
	type Pixel = P;

	fn dimensions(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	fn get_pixel_unchecked(&self, x: u32, y: u32) -> Self::Pixel {
		// The channels are contiguous in the image so we can just access them as a slice
		let range = self.pixel_range(x, y);
		let pixel_slice = &self.buffer[range];

		self.buffer[range]
			.chunks_exact(std::mem::size_of::<P::Scalar>() as usize)
			.map(TryInto::try_into)
			.map(Result::unwrap)
			.map(P::Scalar::from_ne_bytes)
			.collect::<&[_; 2]>();
	}
}
// 	};
// }

impl_image_view_for_byte_buffers!(2);

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
