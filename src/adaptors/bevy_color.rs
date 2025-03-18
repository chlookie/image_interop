use crate::{Color, ConvertColorFrom, formats, spaces};
use bevy_color::ColorToComponents;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl Color<4> for bevy_color::Srgba {
	type Scalar = f32;
	type Format = formats::RGBA;
	type Space = spaces::SRGB;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::LinearRgba {
	type Scalar = f32;
	type Format = formats::RGBA;
	type Space = spaces::LinearRGB;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::Hsla {
	type Scalar = f32;
	type Format = formats::HSLA;
	type Space = spaces::HSL;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::Hsva {
	type Scalar = f32;
	type Format = formats::HSVA;
	type Space = spaces::HSV;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::Hwba {
	type Scalar = f32;
	type Format = formats::HWBA;
	type Space = spaces::HWB;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::Laba {
	type Scalar = f32;
	type Format = formats::LABA;
	type Space = spaces::CieLab;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::Lcha {
	type Scalar = f32;
	type Format = formats::LCHA;
	type Space = spaces::CieLCh;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::Oklaba {
	type Scalar = f32;
	type Format = formats::LABA;
	type Space = spaces::OkLab;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::Oklcha {
	type Scalar = f32;
	type Format = formats::LCHA;
	type Space = spaces::OkLCh;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

impl Color<4> for bevy_color::Xyza {
	type Scalar = f32;
	type Format = formats::XYZA;
	type Space = spaces::CieXYZ;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from_f32_array(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.to_f32_array()
	}
}

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Srgba      { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::Srgba      { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::Srgba      { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::Srgba      { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::LinearRgba { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::LinearRgba { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::LinearRgba { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::LinearRgba { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::Hsla       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Hsla       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::Hsla       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::Hsla       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::Hsla       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::Hsva       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Hsva       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::Hsva       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::Hsva       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::Hsva       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::Hwba       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Hwba       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::Hwba       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::Hwba       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::Hwba       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::Laba       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Laba       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::Laba       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::Laba       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::Laba       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::Laba       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::Laba       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::Laba       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::Laba       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::Lcha       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Lcha       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::Lcha       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::Lcha       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::Lcha       { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::Oklaba     { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Oklaba     { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::Oklaba     { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::Oklaba     { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::Oklcha     { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Oklcha     { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::Oklcha     { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Xyza>       for bevy_color::Oklcha     { fn color_from(color: bevy_color::Xyza)       -> Self { Self::from(color) } }

#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Srgba>      for bevy_color::Xyza       { fn color_from(color: bevy_color::Srgba)      -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::LinearRgba> for bevy_color::Xyza       { fn color_from(color: bevy_color::LinearRgba) -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsla>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Hsla)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hsva>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Hsva)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Hwba>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Hwba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Laba>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Laba)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Lcha>       for bevy_color::Xyza       { fn color_from(color: bevy_color::Lcha)       -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklaba>     for bevy_color::Xyza       { fn color_from(color: bevy_color::Oklaba)     -> Self { Self::from(color) } }
#[rustfmt::skip] impl ConvertColorFrom<4, bevy_color::Oklcha>     for bevy_color::Xyza       { fn color_from(color: bevy_color::Oklcha)     -> Self { Self::from(color) } }
