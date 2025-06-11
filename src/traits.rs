use std::{marker::PhantomData, ops::Range};

use anyhow::{Result, ensure};

use crate::InterleavedLayoutOrder;

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

/// A color channel.
pub trait ColorComponent {}

/// A color space.
pub trait ColorSpace {}

/// A color format.
pub trait ColorFormat<const CHANNELS: Channels> {}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// A generalized color.
pub trait Color<const CHANNELS: Channels>: Copy + Clone {
	/// The scalar type that is used to store each channel in this color.
	type Scalar: ScalarPrimitive;

	/// The format that describes the color channels and their order in this color. For example (Red, Green, Blue, Alpha) or (Hue, Saturation, Value)
	type Format: ColorFormat<CHANNELS>;

	/// The color space in which the color is defined in. Colors in the same color space can be trivially converted to- and from by interchanging their respective components as defined in the color [`Color::Format`].
	type Space: ColorSpace;

	/// Returns a new pixel from an array.
	fn from_array(array: [Self::Scalar; CHANNELS]) -> Self;

	/// Returns this pixel as an array.
	fn to_array(&self) -> [Self::Scalar; CHANNELS];

	/// A view into a slice of contiguous color channels.
	fn as_view(slice: &[Self::Scalar; CHANNELS]) -> PixelView<CHANNELS, Self> {
		PixelView::new(slice)
	}

	/// A mutable view into a slice of contiguous color channels.
	fn as_view_mut(slice: &mut [Self::Scalar; CHANNELS]) -> PixelViewMut<CHANNELS, Self> {
		PixelViewMut::new(slice)
	}
}

/// A color format that can be converted to and from another format.
pub trait ConvertFormatFrom<const CHANNELS: Channels, From, Scalar> {
	/// Convert an array of color channels from one format to another.
	fn convert_array(array: [Scalar; CHANNELS]) -> [Scalar; CHANNELS];
}

/// A color that can be converted to and from another color.
pub trait ConvertColorFrom<const CHANNELS: Channels, From> {
	/// Convert a color from one type to another.
	fn color_from(color: From) -> Self;
}

/// A color that can be converted to and from another color.
pub trait ConvertColorTo<const CHANNELS: Channels, To> {
	/// Convert a color from one type to another.
	fn convert(self) -> To;
}

impl<const CHANNELS: Channels, From, To, Scalar, Space, FormatTo> ConvertColorFrom<CHANNELS, From> for To
where
	From: Color<CHANNELS, Scalar = Scalar, Space = Space>,
	To: Color<CHANNELS, Scalar = Scalar, Space = Space, Format = FormatTo>,
	FormatTo: ConvertFormatFrom<CHANNELS, From::Format, Scalar>,
{
	fn color_from(color: From) -> Self {
		let array_in = color.to_array();
		let array_out = To::Format::convert_array(array_in);
		To::from_array(array_out)
	}
}

impl<const CHANNELS: Channels, From, To> ConvertColorTo<CHANNELS, To> for From
where
	To: ConvertColorFrom<CHANNELS, From>,
{
	fn convert(self) -> To {
		To::color_from(self)
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Describes a generic image layout.
pub trait ImageLayout: Copy + Clone {
	/// The number of color channels in the image.
	fn channels(&self) -> Channels;

	/// The width of the image.
	fn width(&self) -> u32;

	/// The height of the image.
	fn height(&self) -> u32;

	/// The minimum buffer size required to store the image.
	fn minimum_buffer_size(&self) -> usize;

	/// Get the index of a color channel at a given x, y coordinate.
	///
	/// May panic if the channel index or x, y coordinates are out of bounds.
	fn component_index_unchecked(&self, x: u32, y: u32, channel: Channels) -> usize;

	/// Get the index of a color channel at a given x, y coordinate.
	fn component_index(&self, x: u32, y: u32, channel: Channels) -> Result<usize> {
		ensure!(channel < self.channels(), "Channel index out of bounds");
		ensure!(
			x < self.width() && y < self.height(),
			"Pixel x y coordinates out of bounds"
		);

		Ok(self.component_index_unchecked(x, y, channel))
	}
}

/// Describes a generic image layout where the storage of pixels is interleaved (i.e. all channels of a pixel are stored contiguously in memory).
pub trait InterleavedImageLayout: ImageLayout {
	/// Get the index of a pixel at a given x, y coordinate.
	///
	/// May panic if the x, y coordinates are out of bounds.
	fn pixel_index_unchecked(&self, x: u32, y: u32) -> usize;

	/// Get the index of a pixel at a given x, y coordinate.
	fn pixel_index(&self, x: u32, y: u32) -> Result<usize> {
		ensure!(
			x < self.width() && y < self.height(),
			"Pixel x y coordinates out of bounds"
		);

		Ok(self.pixel_index_unchecked(x, y))
	}

	/// Get the range of indices for a pixel at a given x, y coordinate.
	///
	/// May panic if the x, y coordinates are out of bounds.
	fn pixel_range_unchecked(&self, x: u32, y: u32) -> Range<usize> {
		let start = self.pixel_index_unchecked(x, y);
		let end = start + self.channels();
		start..end
	}

	/// Get the range of indices for a pixel at a given x, y coordinate.
	fn pixel_range(&self, x: u32, y: u32) -> Result<Range<usize>> {
		ensure!(
			x < self.width() && y < self.height(),
			"Pixel x y coordinates out of bounds"
		);

		Ok(self.pixel_range_unchecked(x, y))
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

pub trait ChannelShrinkable<T> {
	/// Shrink the layout to a layout with fewer channels by adjusting strides. Fails if the new layout would have more channels.
	fn shrink_channels(self, channels: Channels) -> Result<T>;
}

pub trait Croppable<T> {
	/// Crop the layout to a smaller region.
	fn crop(self, x: u32, y: u32, width: u32, height: u32) -> T;
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// A view into a slice of contiguous color channels.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PixelView<'a, const CHANNELS: Channels, C: Color<CHANNELS>> {
	/// The slice of color channels.
	pub slice: &'a [C::Scalar; CHANNELS],

	_format: PhantomData<C::Format>,
}

impl<'a, const CHANNELS: Channels, C: Color<CHANNELS>> PixelView<'a, CHANNELS, C> {
	/// Create a new view from a slice.
	pub fn new(slice: &'a [C::Scalar; CHANNELS]) -> Self {
		Self {
			slice,
			_format: PhantomData,
		}
	}

	/// Returns the color of the pixel represented by this view.
	pub fn as_color(&self) -> C {
		C::from_array(*self.slice)
	}
}

/// A mutable view into a slice of contiguous color channels.
#[derive(Debug, PartialEq, Eq)]
pub struct PixelViewMut<'a, const CHANNELS: Channels, C: Color<CHANNELS>> {
	/// The slice of color channels.
	pub slice: &'a mut [C::Scalar; CHANNELS],
	_format: PhantomData<C::Format>,
}

impl<'a, const CHANNELS: Channels, C: Color<CHANNELS>> PixelViewMut<'a, CHANNELS, C> {
	/// Create a new mutable view from a slice.
	pub fn new(slice: &'a mut [C::Scalar; CHANNELS]) -> Self {
		Self {
			slice,
			_format: PhantomData,
		}
	}

	/// Returns the color of the pixel represented by this view.
	pub fn as_color(&self) -> C {
		C::from_array(*self.slice)
	}

	/// Set the color of the pixel represented by this view.
	pub fn set_color(&mut self, color: C) {
		*self.slice = color.to_array();
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Trait to inspect an image.
pub trait ImageView<const CHANNELS: Channels> {
	/// The type of each pixel in the image.
	type Pixel: Color<CHANNELS>;

	/// Convenience mthod that returns the number of channels the image has.
	fn channels(&self) -> Channels {
		CHANNELS
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
pub trait ImageViewMut<const CHANNELS: Channels>: ImageView<CHANNELS> {
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
		O: ImageView<CHANNELS, Pixel = Self::Pixel>,
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

pub trait ConvertImage<T> {
	fn convert_image(self) -> T;
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Trait for iterating over the pixels of an image.
pub trait ImageIter<const CHANNELS: Channels> {
	/// The type of each pixel in the image.
	type Pixel: Color<CHANNELS>;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<CHANNELS, Self::Pixel>>;

	/// Returns an iterator over the pixels of the image and their respective coordinates.
	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<CHANNELS, Self::Pixel>)>;
}

/// Trait for mutating the pixels of an image.
pub trait ImageIterMut<const CHANNELS: Channels> {
	/// The type of each pixel in the image.
	type Pixel: Color<CHANNELS>;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<CHANNELS, Self::Pixel>>;

	/// Returns an iterator over the pixels of the image and their respective coordinates.
	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<CHANNELS, Self::Pixel>)>;
}

/// Trait for iterating over the pixels of an image in parallel using rayon.
#[cfg(feature = "rayon")]
pub trait ImageParallelIter<const CHANNELS: Channels> {
	/// The type of each pixel in the image.
	type Pixel: Color<CHANNELS>;

	/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
	fn par_pixels(&self) -> impl rayon::iter::ParallelIterator<Item = PixelView<CHANNELS, Self::Pixel>>;

	/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
	fn par_enumerate_pixels(
		&self,
	) -> impl rayon::iter::ParallelIterator<Item = (u32, u32, PixelView<CHANNELS, Self::Pixel>)>;
}

/// Trait for mutating the pixels of an image in parallel using rayon.
#[cfg(feature = "rayon")]
pub trait ImageParallelIterMut<const CHANNELS: Channels> {
	/// The type of each pixel in the image.
	type Pixel: Color<CHANNELS>;

	/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
	fn par_iter_pixels_mut(&mut self)
	-> impl rayon::iter::ParallelIterator<Item = PixelViewMut<CHANNELS, Self::Pixel>>;

	/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
	fn par_enumerate_pixels_mut(
		&mut self,
	) -> impl rayon::iter::ParallelIterator<Item = (u32, u32, PixelViewMut<CHANNELS, Self::Pixel>)>;
}
