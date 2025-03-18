use std::{fmt::Debug, marker::PhantomData};

use crate::{Channels, Color, ColorFormat, ColorSpace, ScalarPrimitive};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub mod components {
	

	// declare_color_component!(Alpha);
	// declare_color_component!(Red);
	// declare_color_component!(Green);
	// declare_color_component!(Blue);
	// declare_color_component!(Hue);
	// declare_color_component!(Whiteness);
	// declare_color_component!(Blackness);
	// declare_color_component!(Saturation);
	// declare_color_component!(Value);
	// declare_color_component!(Lightness);
	// declare_color_component!(A);
	// declare_color_component!(B);
	// declare_color_component!(Chroma);
	// declare_color_component!(X);
	// declare_color_component!(Y);
	// declare_color_component!(Z);
}

pub mod formats {
	

	// declare_color_format!(RGB: Red, Green, Blue);
	// declare_color_format!(HSL: Hue, Saturation, Lightness);
	// declare_color_format!(HSV: Hue, Saturation, Value);
	// declare_color_format!(HWB: Hue, Whiteness, Blackness);
	// declare_color_format!(LAB: Lightness, A, B);
	// declare_color_format!(LCH: Lightness, Chroma, Hue);
	// declare_color_format!(XYZ: X, Y, Z);
	// declare_color_format!(Y: Y);

	// declare_color_format!(RGBA: Red, Green, Blue, Alpha);
	// declare_color_format!(HSLA: Hue, Saturation, Lightness, Alpha);
	// declare_color_format!(HSVA: Hue, Saturation, Value, Alpha);
	// declare_color_format!(HWBA: Hue, Whiteness, Blackness, Alpha);
	// declare_color_format!(LABA: Lightness, A, B, Alpha);
	// declare_color_format!(LCHA: Lightness, Chroma, Hue, Alpha);
	// declare_color_format!(XYZA: X, Y, Z, Alpha);
	// declare_color_format!(YA: Y, Alpha);
}

pub mod spaces {
	use crate::declare_color_space;

	declare_color_space!(
		UnknownRGB,
		"A color space with sRGB primaries, but unknown whether its transfer function is linear or nonlinear with gamma. The library client needs to make a choice based on any assumptions they can make."
	);

	declare_color_space!(LinearRGB, "The linear-light RGB color space with sRGB primaries.");
	declare_color_space!(SRGB, "The standard RGB color space.");
	declare_color_space!(HSL);
	declare_color_space!(HSV);
	declare_color_space!(HWB);
	declare_color_space!(CieLab);
	declare_color_space!(CieLCh);
	declare_color_space!(CieXYZ);
	declare_color_space!(OkLab);
	declare_color_space!(OkLCh);
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub trait AssumeRGB<const CHANNELS: Channels>
where
	Self: Sized,
{
	fn assume_linear_rgb(self) -> AssumedLinear<Self>;
	fn assume_srgb(self) -> AssumedSrgb<Self>;
}

impl<const CHANNELS: Channels, T> AssumeRGB<CHANNELS> for T
where
	T: Color<CHANNELS, Space = spaces::UnknownRGB>,
{
	fn assume_linear_rgb(self) -> AssumedLinear<Self> {
		AssumedLinear(self)
	}

	fn assume_srgb(self) -> AssumedSrgb<Self> {
		AssumedSrgb(self)
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct AssumedLinear<T>(T)
where
	T: Sized;

impl<const CHANNELS: Channels, T> Color<CHANNELS> for AssumedLinear<T>
where
	T: Color<CHANNELS, Space = spaces::UnknownRGB>,
{
	type Scalar = T::Scalar;
	type Format = T::Format;
	type Space = spaces::LinearRGB;

	fn from_array(array: [Self::Scalar; CHANNELS]) -> Self {
		AssumedLinear(T::from_array(array))
	}

	fn to_array(&self) -> [Self::Scalar; CHANNELS] {
		T::to_array(&self.0)
	}
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct AssumedSrgb<T>(T);

impl<const CHANNELS: Channels, T> Color<CHANNELS> for AssumedSrgb<T>
where
	T: Color<CHANNELS, Space = spaces::UnknownRGB>,
{
	type Scalar = T::Scalar;
	type Format = T::Format;
	type Space = spaces::SRGB;

	fn from_array(array: [Self::Scalar; CHANNELS]) -> Self {
		AssumedSrgb(T::from_array(array))
	}

	fn to_array(&self) -> [Self::Scalar; CHANNELS] {
		T::to_array(&self.0)
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Const generic CHANNELS required because we don't have const_generic_expressions yet
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GenericColor<const CHANNELS: Channels, Format, Scalar, Space> {
	pub color: [Scalar; CHANNELS],
	_format: PhantomData<Format>,
	_space: PhantomData<Space>,
}

impl<const CHANNELS: Channels, Format, Scalar, Space> Default for GenericColor<{ CHANNELS }, Format, Scalar, Space>
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

impl<const CHANNELS: Channels, Format, Scalar, Space> Color<CHANNELS> for GenericColor<CHANNELS, Format, Scalar, Space>
where
	Scalar: ScalarPrimitive,
	Format: Copy + ColorFormat<CHANNELS>,
	Space: Copy + ColorSpace,
{
	type Scalar = Scalar;
	type Format = Format;
	type Space = Space;

	fn from_array(array: [Self::Scalar; CHANNELS]) -> Self {
		Self {
			color: array,
			_format: PhantomData,
			_space: PhantomData,
		}
	}

	fn to_array(&self) -> [Self::Scalar; CHANNELS] {
		self.color
	}
}
