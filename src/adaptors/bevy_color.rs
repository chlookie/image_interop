use crate::{Color, ColorComponents, components::*, declare_color_format};

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

impl Color for bevy_color::Srgba {
	type Scalar = f32;
	type Format = RGBA;
}

impl Color for bevy_color::LinearRgba {
	type Scalar = f32;
	type Format = RGBA;
}

impl Color for bevy_color::Hsla {
	type Scalar = f32;
	type Format = HSLA;
}

impl Color for bevy_color::Hsva {
	type Scalar = f32;
	type Format = HSVA;
}

impl Color for bevy_color::Hwba {
	type Scalar = f32;
	type Format = HWBA;
}

impl Color for bevy_color::Laba {
	type Scalar = f32;
	type Format = LABA;
}

impl Color for bevy_color::Lcha {
	type Scalar = f32;
	type Format = LCHA;
}

impl Color for bevy_color::Oklaba {
	type Scalar = f32;
	type Format = LABA;
}

impl Color for bevy_color::Oklcha {
	type Scalar = f32;
	type Format = LCHA;
}

impl Color for bevy_color::Xyza {
	type Scalar = f32;
	type Format = XYZA;
}

impl<T> ColorComponents for T
where
	T: Color<Scalar = f32> + bevy_color::ColorToComponents,
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
