use crate::{Color, ColorComponents, formats, spaces};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl Color for bevy_color::Srgba {
	type Scalar = f32;
	type Format = formats::RGBA;
	type Space = spaces::SRGB;
}

impl Color for bevy_color::LinearRgba {
	type Scalar = f32;
	type Format = formats::RGBA;
	type Space = spaces::LinearRGB;
}

impl Color for bevy_color::Hsla {
	type Scalar = f32;
	type Format = formats::HSLA;
	type Space = spaces::HSL;
}

impl Color for bevy_color::Hsva {
	type Scalar = f32;
	type Format = formats::HSVA;
	type Space = spaces::HSV;
}

impl Color for bevy_color::Hwba {
	type Scalar = f32;
	type Format = formats::HWBA;
	type Space = spaces::HWB;
}

impl Color for bevy_color::Laba {
	type Scalar = f32;
	type Format = formats::LABA;
	type Space = spaces::CieLab;
}

impl Color for bevy_color::Lcha {
	type Scalar = f32;
	type Format = formats::LCHA;
	type Space = spaces::CieLCh;
}

impl Color for bevy_color::Oklaba {
	type Scalar = f32;
	type Format = formats::LABA;
	type Space = spaces::OkLab;
}

impl Color for bevy_color::Oklcha {
	type Scalar = f32;
	type Format = formats::LCHA;
	type Space = spaces::OkLCh;
}

impl Color for bevy_color::Xyza {
	type Scalar = f32;
	type Format = formats::XYZA;
	type Space = spaces::CieXYZD65;
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
