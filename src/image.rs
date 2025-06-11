use std::{
	marker::{Copy, PhantomData},
	ops::Deref,
};

use anyhow::{Ok, Result, ensure};
use num_traits::FromBytes;

use crate::{AssumedLinear, AssumedSrgb, ChannelShrinkable, Channels, Color, ConvertColorFrom, ImageLayout, spaces};

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
pub struct GenericImage<const CHANNELS: Channels, C, L = PackedLayout, B = Vec<<C as Color<CHANNELS>>::Scalar>>
where
	C: Color<CHANNELS>,
	L: ImageLayout,
{
	/// The backing buffer of the image, which contains the actual samples.
	pub(crate) buffer: B,

	/// The layout of the image, which dictates the size of the image and how to locate pixels in the buffer.
	pub(crate) layout: L,

	/// Some PhatomData for our generic types
	_color: PhantomData<C>,
}

impl<const CHANNELS: Channels, C: Color<CHANNELS>, L: ImageLayout> GenericImage<CHANNELS, C, L, Vec<C::Scalar>> {
	/// Creates a new [`Image`] with a simple contiguous [`Vec`] as a buffer.
	pub fn new(width: u32, height: u32) -> Result<Self>
	where
		PackedLayout: Into<L>,
	{
		let layout: L = PackedLayout::new(CHANNELS, width, height, InterleavedLayoutOrder::RowMajor)?.into();
		let buffer = vec![C::Scalar::default(); CHANNELS * width as usize * height as usize];

		// Sanity check that the layout/buffer dimensions are correct
		debug_assert!(
			buffer.len() == layout.minimum_buffer_size(),
			"The auto-generated buffer is not the exact size for the given layout, this is a bug."
		);

		Ok(Self {
			buffer,
			layout,
			_color: PhantomData,
		})
	}

	/// Creates a new [`Image`] with a simple contiguous [`Vec`] as a buffer.
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

impl<const CHANNELS: Channels, C: Color<CHANNELS>, L: ImageLayout, B> GenericImage<CHANNELS, C, L, B> {
	/// Creates a new [`Image`] instance given a backing buffer and an [`ImageLayout`].
	pub fn from_buffer(buffer: B, layout: L) -> Result<Self>
	where
		B: Deref<Target = [C::Scalar]>,
	{
		// Check that the layout has the correct number of channels
		ensure!(
			layout.channels() == CHANNELS,
			"The given layout has the wrong number of channels for the given color type."
		);

		// Check that the layout/buffer dimensions are correct
		ensure!(
			buffer.len() >= layout.minimum_buffer_size(),
			"The given buffer is too small to fit the entire image as dictated by the given layout."
		);

		Ok(Self {
			buffer,
			layout,
			_color: PhantomData,
		})
	}

	/// Returns the layout of the image.
	pub fn layout(&self) -> L {
		self.layout
	}

	/// Returns the buffer of the image.
	pub fn buffer(&self) -> &B {
		&self.buffer
	}

	/// Returns the buffer of the image.
	pub fn into_buffer(self) -> B {
		self.buffer
	}

	/// Return a transmuted version of the image with a different color type.
	///
	/// Careful; it does not convert the color, it just changes the type as-is regardless of the actual color values. See [`convert_color`] for that instead.
	pub fn transmute_color<DstColor>(self) -> GenericImage<CHANNELS, DstColor, L, B>
	where
		DstColor: Color<CHANNELS, Scalar = C::Scalar>,
	{
		GenericImage {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	/// Shrink the number of channels in the image, and transmute the color type.
	pub fn shrink_channels<const DST_CHANNELS: Channels, DstColor>(
		self,
	) -> Result<GenericImage<DST_CHANNELS, DstColor, LooseLayout, B>>
	where
		DstColor: Color<DST_CHANNELS>,
		L: ChannelShrinkable<LooseLayout>,
	{
		let layout = self.layout.shrink_channels(DST_CHANNELS)?;

		Ok(GenericImage {
			buffer: self.buffer,
			layout,
			_color: PhantomData,
		})
	}

	#[cfg(not(feature = "rayon"))]
	pub fn convert_color<DstColor>(mut self) -> GenericImage<DstColor, L, B>
	where
		C: ColorComponents,
		DstColor: Color<Scalar = C::Scalar> + ColorComponents + ConvertColorFrom<C>,
		Self: ImageIterMut<Pixel = C>,
	{
		for pixel in self.iter_pixels_mut() {
			let color_in = pixel.as_color();
			let color_out = DstColor::color_from(color_in);
			for (dst, src) in pixel.slice.iter_mut().zip(color_out.to_array().as_ref()) {
				*dst = *src
			}
		}

		GenericImage {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	/// Convert the color of the image to a different color type.
	///
	/// If you instead want to transmute the color type without converting the color values, see [`transmute_color`] instead.
	/// TODO: Needs benchmarking to see if it's actually a good idea to use rayon here
	#[cfg(feature = "rayon")]
	pub fn convert_color<DstColor>(mut self) -> GenericImage<CHANNELS, DstColor, L, B>
	where
		DstColor: Color<CHANNELS, Scalar = C::Scalar> + ConvertColorFrom<CHANNELS, C>,
		Self: crate::ImageParallelIterMut<CHANNELS, Pixel = C>,
	{
		use crate::ImageParallelIterMut;
		use rayon::iter::ParallelIterator;

		self.par_iter_pixels_mut().for_each(|pixel| {
			let color_in = pixel.as_color();
			let color_out = DstColor::color_from(color_in);
			for (dst, src) in pixel.slice.iter_mut().zip(color_out.to_array().as_ref()) {
				*dst = *src
			}
		});

		GenericImage {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	/// Assume that the color of the image is linear RGB when it is unknown.
	pub fn assume_linear_rgb(self) -> GenericImage<CHANNELS, AssumedLinear<C>, L, B>
	where
		C: Color<CHANNELS, Space = spaces::UnknownRGB>,
	{
		self.transmute_color()
	}

	/// Assume that the color of the image is sRGB when it is unknown.
	pub fn assume_srgb(self) -> GenericImage<CHANNELS, AssumedSrgb<C>, L, B>
	where
		C: Color<CHANNELS, Space = spaces::UnknownRGB>,
	{
		self.transmute_color()
	}
}
