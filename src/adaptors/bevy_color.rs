use crate::{Color, ColorComponents, ConvertColorFrom, formats, spaces};

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
	type Space = spaces::CieXYZ;
}

use bevy_color::ColorToComponents;
macro_rules! impl_color_components {
	($type:ty) => {
		impl ColorComponents for $type {
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
	};
}

impl_color_components!(bevy_color::Srgba);
impl_color_components!(bevy_color::LinearRgba);
impl_color_components!(bevy_color::Hsla);
impl_color_components!(bevy_color::Hsva);
impl_color_components!(bevy_color::Hwba);
impl_color_components!(bevy_color::Laba);
impl_color_components!(bevy_color::Lcha);
impl_color_components!(bevy_color::Oklaba);
impl_color_components!(bevy_color::Oklcha);
impl_color_components!(bevy_color::Xyza);

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Srgba      { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::Srgba      { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::Srgba      { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::LinearRgba { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::LinearRgba { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::LinearRgba { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::Hsla       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Hsla       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::Hsla       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::Hsla       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::Hsva       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Hsva       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::Hsva       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::Hsva       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::Hwba       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Hwba       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::Hwba       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::Hwba       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::Laba       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Laba       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::Laba       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::Laba       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::Laba       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::Laba       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::Laba       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::Laba       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::Laba       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::Lcha       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Lcha       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::Lcha       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::Lcha       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::Oklaba     { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Oklaba     { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::Oklaba     { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::Oklcha     { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Oklcha     { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::Oklcha     { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Xyza>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Srgba>      for bevy_color::Xyza       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::LinearRgba> for bevy_color::Xyza       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsla>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hsva>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Hwba>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Laba>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Lcha>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklaba>     for bevy_color::Xyza       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<bevy_color::Oklcha>     for bevy_color::Xyza       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
