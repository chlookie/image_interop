use bevy_color::{ColorToComponents, Hsla, Hsva, Hwba, Laba, Lcha, LinearRgba, Oklaba, Oklcha, Srgba, Xyza};

use crate::{Pixel, PixelToComponents, component::*, declare_color_format};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

declare_color_format!(RGBA: Red, Green, Blue, Alpha);
declare_color_format!(HSLA: Hue, Saturation, Lightness, Alpha);
declare_color_format!(HSVA: Hue, Saturation, Value, Alpha);
declare_color_format!(HWBA: Hue, Whiteness, Blackness, Alpha);
declare_color_format!(LABA: Lightness, A, B, Alpha);
declare_color_format!(LCHA: Lightness, Chroma, Hue, Alpha);
declare_color_format!(XYZA: X, Y, Z, Alpha);

impl Pixel for Srgba {
	type Scalar = f32;
	type Format = RGBA;
}

impl Pixel for LinearRgba {
	type Scalar = f32;
	type Format = RGBA;
}

impl Pixel for Hsla {
	type Scalar = f32;
	type Format = HSLA;
}

impl Pixel for Hsva {
	type Scalar = f32;
	type Format = HSVA;
}

impl Pixel for Hwba {
	type Scalar = f32;
	type Format = HWBA;
}

impl Pixel for Laba {
	type Scalar = f32;
	type Format = LABA;
}

impl Pixel for Lcha {
	type Scalar = f32;
	type Format = LCHA;
}

impl Pixel for Oklaba {
	type Scalar = f32;
	type Format = LABA;
}

impl Pixel for Oklcha {
	type Scalar = f32;
	type Format = LCHA;
}

impl Pixel for Xyza {
	type Scalar = f32;
	type Format = XYZA;
}

impl<T> PixelToComponents for T
where
	T: Pixel<Scalar = f32> + ColorToComponents,
{
	type Tuple = (f32, f32, f32, f32);
	type Array = [f32; 4];

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
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> Self::Array {
		self.to_f32_array()
	}
}
