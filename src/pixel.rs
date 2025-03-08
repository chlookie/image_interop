use std::{fmt::Debug, marker::PhantomData};

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

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub mod component {
	use super::*;

	pub trait ColorComponent {}

	#[rustfmt::skip]
	macro_rules! declare_color_component {
		($type:ident, $name:ident) => {paste::paste!{
		
			#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
			pub struct $type;
			impl ColorComponent for $type {}
			
			pub trait [<Has $type>]<Scalar> {
				fn component(slice: &[Scalar]) -> Scalar;
				fn component_mut(slice: &mut [Scalar]) -> &mut Scalar;
			}
			
			#[allow(dead_code)]
			impl<'a, P: Pixel> PixelView<'a, P>
			where
				P::Format: [<Has $type>]<P::Scalar>,
			{
				pub fn $name(&self) -> P::Scalar {
					<P::Format as [<Has $type>]<P::Scalar>>::component(self.slice)
				}
			}
			
			#[allow(dead_code)]
			impl<'a, P: Pixel> PixelViewMut<'a, P>
			where
				P::Format: [<Has $type>]<P::Scalar>,
			{
				pub fn $name(&self) -> P::Scalar {
					<P::Format as [<Has $type>]<P::Scalar>>::component(self.slice)
				}
				
				pub fn [<set_ $name>](&mut self, $name: P::Scalar) {
					*<P::Format as [<Has $type>]<P::Scalar>>::component_mut(self.slice) = $name
				}
			}
		}};
	}

	declare_color_component!(Alpha, alpha);
	declare_color_component!(Red, red);
	declare_color_component!(Green, green);
	declare_color_component!(Blue, blue);
	declare_color_component!(Hue, hue);
	declare_color_component!(Whiteness, whiteness);
	declare_color_component!(Blackness, blackness);
	declare_color_component!(Saturation, saturation);
	declare_color_component!(Value, value);
	declare_color_component!(Lightness, lightness);
	declare_color_component!(A, a);
	declare_color_component!(B, b);
	declare_color_component!(Chroma, chroma);
	declare_color_component!(X, x);
	declare_color_component!(Y, y);
	declare_color_component!(Z, z);
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub type Channels = usize;

pub trait ColorFormat<S: ScalarPrimitive> {
	const CHANNELS: Channels;
}

#[macro_export]
macro_rules! declare_color_format {
	($alias:ident: $($channels:ty),+) => {

		#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
		pub struct $alias;

		impl<Scalar: $crate::ScalarPrimitive> $crate::ColorFormat<Scalar> for $alias {
			const CHANNELS: $crate::Channels = declare_color_format!(@count_channels $($channels,)*);
		}

		declare_color_format!(@iter_channels 0, $alias: $($channels,)+);
	};

	(@iter_channels $index:expr, $alias:ident: ) => {};

	(@iter_channels $index:expr, $alias:ident: $channel:ty, $($channels:ty,)*) => {paste::paste!{
		impl<Scalar: $crate::ScalarPrimitive> $crate::component::[<Has $channel>]<Scalar> for $alias {
			fn component(slice: &[Scalar]) -> Scalar {
				slice[$index]
			}

			fn component_mut(slice: &mut [Scalar]) -> &mut Scalar {
				&mut slice[$index]
			}
		}

		declare_color_format!(@iter_channels ($index+1), $alias: $($channels,)*);
	}};

	(@count_channels $channel:ty, $($channels:ty,)*) => {1 + declare_color_format!(@count_channels $($channels,)*)};
	(@count_channels ) => {0};
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

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PixelView<'a, P: Pixel> {
	slice: &'a [P::Scalar],
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
	slice: &'a mut [P::Scalar],
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
