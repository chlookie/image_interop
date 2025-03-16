use std::{marker::PhantomData, ops::Deref};

use anyhow::{Ok, Result, ensure};
use num_traits::FromBytes;

use crate::{AssumedLinear, AssumedSrgb, Channels, Color, ColorComponents, ConvertColorFrom, ImageLayout, spaces};

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
pub struct GenericImage<C: Color, Layout: ImageLayout = PackedLayout, Buffer = Vec<<C as Color>::Scalar>> {
	/// The backing buffer of the image, which contains the actual samples.
	pub(crate) buffer: Buffer,

	/// The layout of the image, which dictates the size of the image and how to locate pixels in the buffer.
	pub(crate) layout: Layout,

	/// Some PhatomData for our generic types
	_color: PhantomData<C>,
}

impl<C: Color, L: ImageLayout> GenericImage<C, L, Vec<C::Scalar>> {
	/// Creates a new [`Image`] with a simple contiguous [`Vec`] as a buffer.
	pub fn new(width: u32, height: u32) -> Result<Self>
	where
		PackedLayout: Into<L>,
	{
		let layout: L = PackedLayout::new(width, height, InterleavedLayoutOrder::RowMajor)?.into();
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

impl<C: Color, L: ImageLayout, B> GenericImage<C, L, B> {
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
	pub fn convert_color<C2>(mut self) -> GenericImage<C2, L, B>
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

		GenericImage {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	/// TODO: Needs benchmarking to see if it's actually a good idea to use rayon here
	#[cfg(feature = "rayon")]
	pub fn convert_color<C2>(mut self) -> GenericImage<C2, L, B>
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

		GenericImage {
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

	pub fn assume_linear_rgb(self) -> GenericImage<AssumedLinear<C>, L, B>
	where
		C: Color<Space = spaces::UnknownRGB>,
	{
		GenericImage {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}

	pub fn assume_srgb(self) -> GenericImage<AssumedSrgb<C>, L, B>
	where
		C: Color<Space = spaces::UnknownRGB>,
	{
		GenericImage {
			buffer: self.buffer,
			layout: self.layout,
			_color: PhantomData,
		}
	}
}
