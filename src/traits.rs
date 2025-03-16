use std::{marker::PhantomData, ops::Range};

use anyhow::{Result, ensure};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// A scalar primitive that can be used to store color channels.
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
pub const MAX_CHANNELS: Channels = 256;

/// A color channel.
pub trait ColorComponent {}

/// A color space.
pub trait ColorSpace {}

/// A color format.
pub trait ColorFormat {
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
			"Given slice doesn't match the number of channels for the color type"
		);
	};
}

/// A generalized color.
pub trait Color: Copy + Clone {
	/// The scalar type that is used to store each channel in this color.
	type Scalar: ScalarPrimitive;

	/// The format that describes the color channels and their order in this color. For example (Red, Green, Blue, Alpha) or (Hue, Saturation, Value)
	type Format: ColorFormat;

	/// The color space in which the color is defined in. Colors in the same color space can be trivially converted to- and from by interchanging their respective components as defined in the color [`Color::Format`].
	type Space: ColorSpace;

	/// The number of channels in this color.
	const CHANNELS: Channels = Self::Format::CHANNELS;

	/// A view into a slice of contiguous color channels.
	fn as_view(slice: &[Self::Scalar]) -> Result<PixelView<Self>> {
		PixelView::new(slice)
	}

	/// A mutable view into a slice of contiguous color channels.
	fn as_view_mut(slice: &mut [Self::Scalar]) -> Result<PixelViewMut<Self>> {
		PixelViewMut::new(slice)
	}

	/// A view into a slice of contiguous color channels.
	///
	/// May panic if the slice is of the wrong length.
	fn as_view_unchecked(slice: &[Self::Scalar]) -> PixelView<Self> {
		PixelView::new_unchecked(slice)
	}

	/// A mutable view into a slice of contiguous color channels.
	///
	/// May panic if the slice is of the wrong length.
	fn as_view_mut_unchecked(slice: &mut [Self::Scalar]) -> PixelViewMut<Self> {
		PixelViewMut::new_unchecked(slice)
	}
}

pub trait ColorComponents: Color {
	/// The tuple type that represents the channels of this color.
	type Tuple;

	/// The array type that represents the channels of this color.
	type Array: AsRef<[Self::Scalar]>;

	/// Returns a new pixel from a slice.
	fn from_slice(slice: &[Self::Scalar]) -> Result<Self> {
		check_channels!(slice, Self::CHANNELS);

		Ok(Self::from_slice_unchecked(slice))
	}

	/// Returns a new pixel from a slice.
	///
	/// May panic if the slice is of the wrong length.
	fn from_slice_unchecked(slice: &[Self::Scalar]) -> Self;

	/// Returns a new pixel from a tuple.
	fn from_tuple(tuple: Self::Tuple) -> Self;

	/// Returns this pixel as a tuple.
	fn to_tuple(&self) -> Self::Tuple;

	/// Returns a new pixel from an array.
	fn from_array(array: Self::Array) -> Self;

	/// Returns this pixel as an array.
	fn to_array(&self) -> Self::Array;
}

/// A color format that can be converted to and from another format.
pub trait ConvertFormatFrom<From, Scalar, const CHANNELS: Channels> {
	/// Convert a slice of color channels from one format to another.
	fn convert_slice(slice: &[Scalar]) -> [Scalar; CHANNELS];
}

/// A color that can be converted to and from another color.
pub trait ConvertColorFrom<From> {
	/// Convert a color from one type to another.
	fn color_from(color: From) -> Self;
}

impl<From, To, Scalar, Space, FormatTo, const CHANNELS: Channels> ConvertColorFrom<From> for To
where
	From: Color<Scalar = Scalar, Space = Space> + ColorComponents,
	To: Color<Scalar = Scalar, Space = Space, Format = FormatTo> + ColorComponents<Array = [Scalar; CHANNELS]>,
	FormatTo: ConvertFormatFrom<From::Format, Scalar, { CHANNELS }>,
{
	fn color_from(color: From) -> Self {
		let array_in = color.to_array();
		let array_out = To::Format::convert_slice(array_in.as_ref());
		To::from_array(array_out)
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes a generic image layout.
pub trait ImageLayout: Copy + Clone {
	/// The width of the image.
	fn width(&self) -> u32;

	/// The height of the image.
	fn height(&self) -> u32;

	/// The minimum buffer size required to store the image.
	fn minimum_buffer_size(&self, channels: Channels) -> usize;

	/// Get the index of a color channel at a given x, y coordinate.
	///
	/// May panic if the channel index or x, y coordinates are out of bounds.
	fn color_channel_index_unchecked(&self, channels: Channels, x: u32, y: u32, channel: Channels) -> usize;

	/// Get the index of a color channel at a given x, y coordinate.
	fn color_channel_index(&self, channels: Channels, x: u32, y: u32, channel: Channels) -> Result<usize> {
		ensure!(channel < channels, "Channel index out of bounds");
		ensure!(
			x < self.width() && y < self.height(),
			"Pixel x y coordinates out of bounds"
		);

		Ok(self.color_channel_index_unchecked(channels, x, y, channel))
	}
}

/// Describes a generic image layout where the storage of pixels is interleaved (i.e. all channels of a pixel are stored contiguously in memory).
pub trait InterleavedImageLayout: ImageLayout {
	/// Get the index of a pixel at a given x, y coordinate.
	///
	/// May panic if the x, y coordinates are out of bounds.
	fn pixel_index_unchecked(&self, channels: Channels, x: u32, y: u32) -> usize;

	/// Get the index of a pixel at a given x, y coordinate.
	fn pixel_index(&self, channels: Channels, x: u32, y: u32) -> Result<usize> {
		ensure!(
			x < self.width() && y < self.height(),
			"Pixel x y coordinates out of bounds"
		);

		Ok(self.pixel_index_unchecked(channels, x, y))
	}

	/// Get the range of indices for a pixel at a given x, y coordinate.
	///
	/// May panic if the x, y coordinates are out of bounds.
	fn pixel_range_unchecked(&self, channels: Channels, x: u32, y: u32) -> Range<usize> {
		let start = self.pixel_index_unchecked(channels, x, y);
		let end = start + channels;
		start..end
	}

	/// Get the range of indices for a pixel at a given x, y coordinate.
	fn pixel_range(&self, channels: Channels, x: u32, y: u32) -> Result<Range<usize>> {
		ensure!(
			x < self.width() && y < self.height(),
			"Pixel x y coordinates out of bounds"
		);

		Ok(self.pixel_range_unchecked(channels, x, y))
	}

	/// Get the layout storage order.
	fn order(&self) -> InterleavedLayoutOrder;

	/// Returns wehter the layout is in row major order.
	fn is_row_major(&self) -> bool {
		self.order() == InterleavedLayoutOrder::RowMajor
	}

	/// Returns wehter the layout is in column major order.
	fn is_column_major(&self) -> bool {
		self.order() == InterleavedLayoutOrder::ColumnMajor
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// A view into a slice of contiguous color channels.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PixelView<'a, C: Color> {
	/// The slice of color channels.
	pub slice: &'a [C::Scalar],

	_format: PhantomData<C::Format>,
}

impl<'a, C: Color> PixelView<'a, C> {
	/// Create a new view from a slice.
	pub fn new(slice: &'a [C::Scalar]) -> Result<Self> {
		check_channels!(slice, C::CHANNELS);
		Ok(Self::new_unchecked(slice))
	}

	/// Create a new view from a slice.
	///
	/// May panic if the slice is of the wrong length.
	pub fn new_unchecked(slice: &'a [C::Scalar]) -> Self {
		PixelView {
			slice,
			_format: PhantomData,
		}
	}

	/// Returns the color of the pixel represented by this view.
	pub fn as_color(&self) -> C
	where
		C: ColorComponents,
	{
		C::from_slice_unchecked(self.slice)
	}
}

/// A mutable view into a slice of contiguous color channels.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct PixelViewMut<'a, C: Color> {
	/// The slice of color channels.
	pub slice: &'a mut [C::Scalar],
	_format: PhantomData<C::Format>,
}

impl<'a, C: Color> PixelViewMut<'a, C> {
	/// Create a new mutable view from a slice.
	pub fn new(slice: &'a mut [C::Scalar]) -> Result<Self> {
		check_channels!(slice, C::Format::CHANNELS);
		Ok(Self::new_unchecked(slice))
	}

	/// Create a new mutable view from a slice.
	///
	/// May panic if the slice is of the wrong length.
	pub fn new_unchecked(slice: &'a mut [C::Scalar]) -> Self {
		PixelViewMut {
			slice,
			_format: PhantomData,
		}
	}

	/// Returns the color of the pixel represented by this view.
	pub fn as_color(&self) -> C
	where
		C: ColorComponents,
	{
		C::from_slice_unchecked(self.slice)
	}

	/// Set the color of the pixel represented by this view.
	pub fn set_color(&mut self, color: C)
	where
		C: ColorComponents,
	{
		self.slice.copy_from_slice(color.to_array().as_ref());
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Trait to inspect an image.
pub trait ImageView {
	/// The type of each pixel in the image.
	type Pixel: Color;

	// The number of channels the image has.
	const CHANNELS: Channels;

	/// COnvenience mthod that returns the number of channels the image has.
	fn channels(&self) -> Channels {
		Self::CHANNELS
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
	/// May panic if the x, y coordinates are out of bounds.
	fn get_pixel_unchecked(&self, x: u32, y: u32) -> Self::Pixel;
}

/// A trait for manipulating images.
pub trait ImageViewMut: ImageView {
	/// Put a pixel at location (x, y). Indexed from top left.
	fn put_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) -> Result<()> {
		ensure!(
			x < self.width() && y < self.height(),
			"Pixel x y coordinates out of bounds"
		);

		self.put_pixel_unchecked(x, y, pixel);
		Ok(())
	}

	/// Put a pixel at location (x, y). Indexed from top left.
	///
	/// May panic if the x, y coordinates are out of bounds.
	fn put_pixel_unchecked(&mut self, x: u32, y: u32, pixel: Self::Pixel);

	/// Copies all of the pixels from another image into this image.
	///
	/// Both images must have matching sizes.
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

/// Trait for iterating over the pixels of an image.
pub trait ImageIter {
	/// The type of each pixel in the image.
	type Pixel: Color;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<Self::Pixel>>;

	/// Returns an iterator over the pixels of the image and their respective coordinates.
	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<Self::Pixel>)>;
}

/// Trait for mutating the pixels of an image.
pub trait ImageIterMut {
	/// The type of each pixel in the image.
	type Pixel: Color;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<Self::Pixel>>;

	/// Returns an iterator over the pixels of the image and their respective coordinates.
	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<Self::Pixel>)>;
}

#[cfg(feature = "rayon")]
mod par_iter {
	use rayon::iter::ParallelIterator;

	use super::*;

	/// Trait for iterating over the pixels of an image in parallel using rayon.
	pub trait ImageParallelIter {
		/// The type of each pixel in the image.
		type Pixel: Color;

		/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
		fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<Self::Pixel>>;

		/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
		fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<Self::Pixel>)>;
	}

	/// Trait for mutating the pixels of an image in parallel using rayon.
	pub trait ImageParallelIterMut {
		/// The type of each pixel in the image.
		type Pixel: Color;

		/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
		fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<Self::Pixel>>;

		/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
		fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<Self::Pixel>)>;
	}
}

#[cfg(feature = "rayon")]
pub use par_iter::*;

use crate::InterleavedLayoutOrder;
