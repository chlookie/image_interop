use std::{
	marker::PhantomData,
	ops::{Deref, DerefMut, Range},
};

use anyhow::{Context, Ok, Result, ensure};

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
	pub fn row_major_packed(channels: usize, width: u32, height: u32) -> Self {
		ImageLayout {
			width,
			height,
			stride_x: channels as u32,
			stride_y: channels as u32 * width,
		}
	}

	pub fn column_major_packed(channels: usize, width: u32, height: u32) -> Self {
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

	pub fn image_buffer_length(&self, channels: usize) -> Option<usize> {
		// Since we are using strides, can't just do width*height
		let size_x = self.stride_x.checked_mul(self.width)?;
		let size_y = self.stride_y.checked_mul(self.height)?;

		// So take the biggest one instead (i.e. the major dimension)
		let size = size_x.max(size_y) as usize;
		size.checked_mul(channels)
	}

	pub fn image_buffer_fits(&self, channels: usize, length: usize) -> bool {
		self.image_buffer_length(channels) == Some(length)
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

/// A custom image type compatible with/based on the [image] crate.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Image<P: Pixel, Buffer = Vec<<P as Pixel>::Scalar>> {
	/// The backing buffer of the image, which contains the actual samples.
	pub(crate) buffer: Buffer,

	/// The layout of the image, which dictates the size of the image and how to locate pixels in the buffer.
	pub(crate) layout: ImageLayout,

	/// Some PhatomData for the Pixel type.
	_pixel: PhantomData<P>,
}

impl<P: Pixel> Image<P, Vec<P::Scalar>> {
	/// Creates a new [`Image`] with a simple empty contiguous [`Vec`] as a buffer.
	pub fn new(width: u32, height: u32) -> Self {
		let buffer = vec![P::Scalar::default(); Self::CHANNELS * width as usize * height as usize];

		let layout = ImageLayout::row_major_packed(Self::CHANNELS, width, height);

		// Sanity check that the layout/buffer dimensions are correct
		debug_assert!(
			layout.image_buffer_fits(Self::CHANNELS, buffer.len()),
			"Buffer has wrong size for the given layout"
		);

		Self {
			buffer,
			layout,
			_pixel: PhantomData,
		}
	}
}

impl<Buffer, P: Pixel> Image<P, Buffer> {
	pub const CHANNELS: Channels = P::CHANNELS;

	/// Returns the number of channels the color format of the image has.
	pub const fn channels(&self) -> Channels {
		Self::CHANNELS
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

	fn pixel_range(&self, x: u32, y: u32) -> Range<usize> {
		let index = self.layout.index(x, y);
		let channels = self.channels() as usize;
		index..index + channels
	}
}

impl<Buffer, P> Image<P, Buffer>
where
	P: Pixel,
	Buffer: DerefMut<Target = [P::Scalar]>,
{
	/// Creates a new [`Image`] instance given a backing buffer and an [`ImageLayout`].
	pub fn from_buffer(buffer: Buffer, layout: ImageLayout) -> Result<Self> {
		ensure!(
			layout.image_buffer_fits(Self::CHANNELS, buffer.len()),
			"The given buffer is not of the right size for the given layout!"
		);

		Ok(Self {
			buffer,
			layout,
			_pixel: PhantomData,
		})
	}

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
	fn get_pixel(&self, x: u32, y: u32) -> Result<Self::Pixel>;

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

impl<Buffer, P> ImageView for Image<P, Buffer>
where
	P: Pixel + PixelToComponents,
	Buffer: Deref<Target = [P::Scalar]>,
{
	type Pixel = P;

	fn dimensions(&self) -> (u32, u32) {
		(self.width(), self.height())
	}

	fn get_pixel(&self, x: u32, y: u32) -> Result<Self::Pixel> {
		// The channels are contiguous in the image so we can just access them as a slice
		let range = self.pixel_range(x, y);
		let pixel_slice = &self.buffer.get(range).context("Pixel x y coordinates out of bounds")?;

		Ok(P::from_slice_unchecked(pixel_slice))
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
