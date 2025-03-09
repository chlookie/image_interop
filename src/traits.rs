use std::marker::PhantomData;

use anyhow::{Result, ensure};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub trait ScalarPrimitive: Copy + Clone + Default {}

impl ScalarPrimitive for u8 {}
impl ScalarPrimitive for u16 {}
impl ScalarPrimitive for u32 {}
impl ScalarPrimitive for u64 {}
impl ScalarPrimitive for u128 {}
impl ScalarPrimitive for usize {}

impl ScalarPrimitive for i8 {}
impl ScalarPrimitive for i16 {}
impl ScalarPrimitive for i32 {}
impl ScalarPrimitive for i64 {}
impl ScalarPrimitive for i128 {}
impl ScalarPrimitive for isize {}

impl ScalarPrimitive for f32 {}
impl ScalarPrimitive for f64 {}

#[cfg(feature = "f16")]
impl ScalarPrimitive for half::f16 {}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub type Channels = usize;

pub trait ColorComponent {}

pub trait ColorFormat<S: ScalarPrimitive> {
	const CHANNELS: Channels;
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

macro_rules! check_channels {
	($id:ident, $channels:expr) => {
		ensure!(
			$id.len() == $channels,
			"Given slice doesn't match the number of channels for the pixel type"
		);
	};
}

/// A generalized pixel.
pub trait Pixel: Copy + Clone {
	/// The scalar type that is used to store each channel in this pixel.
	type Scalar: ScalarPrimitive;

	/// The format of the channels in this pixel. For example (Red, Green, Blue, Alpha) or (Hue, Saturation, Value)
	type Format: ColorFormat<Self::Scalar>;

	const CHANNELS: Channels = Self::Format::CHANNELS;

	fn as_view(slice: &[Self::Scalar]) -> Result<PixelView<Self>> {
		PixelView::new(slice)
	}

	fn as_view_mut(slice: &mut [Self::Scalar]) -> Result<PixelViewMut<Self>> {
		PixelViewMut::new(slice)
	}

	fn as_view_unchecked(slice: &[Self::Scalar]) -> PixelView<Self> {
		PixelView::new_unchecked(slice)
	}

	fn as_view_mut_unchecked(slice: &mut [Self::Scalar]) -> PixelViewMut<Self> {
		PixelViewMut::new_unchecked(slice)
	}
}

pub trait PixelToComponents: Pixel {
	type Tuple;
	type Array: AsRef<[Self::Scalar]>;

	/// Returns a new pixel from a slice, and checks if the slice size is correct.
	fn from_slice(slice: &[Self::Scalar]) -> Result<Self> {
		check_channels!(slice, Self::CHANNELS);

		Ok(Self::from_slice_unchecked(slice))
	}

	/// Returns a new pixel from a slice, but might panic if the slice is of the wrong length.
	fn from_slice_unchecked(slice: &[Self::Scalar]) -> Self;

	fn from_tuple(tuple: Self::Tuple) -> Self;

	fn to_tuple(&self) -> Self::Tuple;

	fn from_array(array: Self::Array) -> Self;

	fn to_array(&self) -> Self::Array;
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PixelView<'a, P: Pixel> {
	pub slice: &'a [P::Scalar],
	_format: PhantomData<P::Format>,
}

impl<'a, P: Pixel> PixelView<'a, P> {
	pub fn new(slice: &'a [P::Scalar]) -> Result<Self> {
		check_channels!(slice, P::CHANNELS);
		Ok(Self::new_unchecked(slice))
	}

	pub fn new_unchecked(slice: &'a [P::Scalar]) -> Self {
		PixelView {
			slice,
			_format: PhantomData,
		}
	}

	pub fn as_pixel(&self) -> P
	where
		P: PixelToComponents,
	{
		P::from_slice_unchecked(self.slice)
	}
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PixelViewMut<'a, P: Pixel> {
	pub slice: &'a mut [P::Scalar],
	_format: PhantomData<P::Format>,
}

impl<'a, P: Pixel> PixelViewMut<'a, P> {
	pub fn new(slice: &'a mut [P::Scalar]) -> Result<Self> {
		check_channels!(slice, P::Format::CHANNELS);
		Ok(Self::new_unchecked(slice))
	}

	pub fn new_unchecked(slice: &'a mut [P::Scalar]) -> Self {
		PixelViewMut {
			slice,
			_format: PhantomData,
		}
	}

	pub fn as_pixel(&self) -> P
	where
		P: PixelToComponents,
	{
		P::from_slice_unchecked(self.slice)
	}

	pub fn set_pixel(&mut self, pixel: P)
	where
		P: PixelToComponents,
	{
		self.slice.copy_from_slice(pixel.to_array().as_ref());
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

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub trait ImageIter {
	/// The type of pixel.
	type Pixel: Pixel;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<Self::Pixel>>;

	/// Returns an iterator over the pixels of the image and their respective coordinates.
	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<Self::Pixel>)>;
}

pub trait ImageIterMut {
	/// The type of pixel.
	type Pixel: Pixel;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<Self::Pixel>>;

	/// Returns an iterator over the pixels of the image and their respective coordinates.
	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<Self::Pixel>)>;
}

#[cfg(feature = "rayon")]
mod par_iter {
	use rayon::iter::ParallelIterator;

	use super::*;

	pub trait ImageParallelIter {
		/// The type of pixel.
		type Pixel: Pixel;

		/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
		fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<Self::Pixel>>;

		/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
		fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<Self::Pixel>)>;
	}

	pub trait ImageParallelIterMut {
		/// The type of pixel.
		type Pixel: Pixel;

		/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
		fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<Self::Pixel>>;

		/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
		fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<Self::Pixel>)>;
	}
}

#[cfg(feature = "rayon")]
pub use par_iter::*;
