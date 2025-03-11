use std::{fmt::Debug, marker::PhantomData};

use crate::{Channels, Color, ColorComponents, ColorFormat, ColorSpace, ScalarPrimitive};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub mod components {
	use crate::declare_color_component;

	declare_color_component!(Alpha);
	declare_color_component!(Red);
	declare_color_component!(Green);
	declare_color_component!(Blue);
	declare_color_component!(Hue);
	declare_color_component!(Whiteness);
	declare_color_component!(Blackness);
	declare_color_component!(Saturation);
	declare_color_component!(Value);
	declare_color_component!(Lightness);
	declare_color_component!(A);
	declare_color_component!(B);
	declare_color_component!(Chroma);
	declare_color_component!(X);
	declare_color_component!(Y);
	declare_color_component!(Z);
}

pub mod formats {
	use crate::declare_color_format;

	declare_color_format!(RGB: Red, Green, Blue);
	declare_color_format!(HSL: Hue, Saturation, Lightness);
	declare_color_format!(HSV: Hue, Saturation, Value);
	declare_color_format!(HWB: Hue, Whiteness, Blackness);
	declare_color_format!(LAB: Lightness, A, B);
	declare_color_format!(LCH: Lightness, Chroma, Hue);
	declare_color_format!(XYZ: X, Y, Z);

	declare_color_format!(RGBA: Red, Green, Blue, Alpha);
	declare_color_format!(HSLA: Hue, Saturation, Lightness, Alpha);
	declare_color_format!(HSVA: Hue, Saturation, Value, Alpha);
	declare_color_format!(HWBA: Hue, Whiteness, Blackness, Alpha);
	declare_color_format!(LABA: Lightness, A, B, Alpha);
	declare_color_format!(LCHA: Lightness, Chroma, Hue, Alpha);
	declare_color_format!(XYZA: X, Y, Z, Alpha);
}

pub mod spaces {
	use crate::declare_color_space;

	declare_color_space!(
		RGB,
		"A color space with sRGB primaries, but unknown whether its transfer function is linear or nonlinear with gamma. The library client needs to make a choice based on any assumptions they can make."
	);

	declare_color_space!(LinearRGB, "The linear-light RGB color space with sRGB primaries.");
	declare_color_space!(SRGB, "The standard RGB color space.");
	declare_color_space!(HSL);
	declare_color_space!(HSV);
	declare_color_space!(HWB);
	declare_color_space!(CieLab);
	declare_color_space!(CieLCh);
	declare_color_space!(CieXYZD65);
	declare_color_space!(OkLab);
	declare_color_space!(OkLCh);
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Const generic CHANNELS required because we don't have const_generic_expressions yet
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StaticColor<const CHANNELS: Channels, Format, Scalar, Space> {
	pub color: [Scalar; CHANNELS],
	_format: PhantomData<Format>,
	_space: PhantomData<Space>,
}

impl<const CHANNELS: Channels, Format, Scalar, Space> Default for StaticColor<{ CHANNELS }, Format, Scalar, Space>
where
	Scalar: Default + Copy,
{
	fn default() -> Self {
		Self {
			color: [Default::default(); CHANNELS],
			_format: Default::default(),
			_space: Default::default(),
		}
	}
}

impl<const CHANNELS: Channels, Format, Scalar, Space> Color for StaticColor<{ CHANNELS }, Format, Scalar, Space>
where
	Scalar: ScalarPrimitive,
	Format: Copy + ColorFormat,
	Space: Copy + ColorSpace,
{
	type Scalar = Scalar;
	type Format = Format;
	type Space = Space;
}

macro_rules! impl_color_components_for_generic_color {
	($channels:expr, $tuple:ty) => {
		impl<Format, Scalar, Space> ColorComponents for StaticColor<{ $channels }, Format, Scalar, Space>
		where
			Scalar: ScalarPrimitive,
			Format: Copy + ColorFormat,
			Space: Copy + ColorSpace,
		{
			type Tuple = $tuple;
			type Array = [Scalar; $channels];

			fn from_slice_unchecked(slice: &[Self::Scalar]) -> Self {
				Self::from_array(slice.try_into().unwrap())
			}

			fn from_tuple(tuple: Self::Tuple) -> Self {
				Self::from_array(tuple.into())
			}

			fn to_tuple(&self) -> Self::Tuple {
				self.to_array().into()
			}

			fn from_array(array: Self::Array) -> Self {
				Self {
					color: array,
					_format: PhantomData,
					_space: PhantomData,
				}
			}

			fn to_array(&self) -> Self::Array {
				self.color
			}
		}
	};
}

#[rustfmt::skip] impl_color_components_for_generic_color!(1,  (Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(2,  (Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(3,  (Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(4,  (Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(5,  (Scalar, Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(6,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(7,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(8,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(9,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(10, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(11, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(12, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,));
