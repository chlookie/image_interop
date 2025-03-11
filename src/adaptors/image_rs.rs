use crate::{Color, ScalarPrimitive, formats, spaces};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<Scalar: image::Primitive + ScalarPrimitive> Color for image::Rgb<Scalar> {
	type Scalar = Scalar;
	type Format = formats::RGB;
	type Space = spaces::SRGB;
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color for image::Rgba<Scalar> {
	type Scalar = Scalar;
	type Format = formats::RGBA;
	type Space = spaces::SRGB;
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color for image::Luma<Scalar> {
	type Scalar = Scalar;
	type Format = formats::Y;
	type Space = spaces::CieXYZD65;
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color for image::LumaA<Scalar> {
	type Scalar = Scalar;
	type Format = formats::YA;
	type Space = spaces::CieXYZD65;
}
