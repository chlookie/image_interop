use crate::{Color, ScalarPrimitive, formats, spaces};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<Scalar: image::Primitive + ScalarPrimitive> Color<3> for image::Rgb<Scalar> {
	type Scalar = Scalar;
	type Format = formats::RGB;
	type Space = spaces::SRGB;

	fn from_array(array: [Self::Scalar; 3]) -> Self {
		Self::from(array)
	}

	fn to_array(&self) -> [Self::Scalar; 3] {
		self.0
	}
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color<4> for image::Rgba<Scalar> {
	type Scalar = Scalar;
	type Format = formats::RGBA;
	type Space = spaces::SRGB;

	fn from_array(array: [Self::Scalar; 4]) -> Self {
		Self::from(array)
	}

	fn to_array(&self) -> [Self::Scalar; 4] {
		self.0
	}
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color<1> for image::Luma<Scalar> {
	type Scalar = Scalar;
	type Format = formats::Y;
	type Space = spaces::CieXYZ;

	fn from_array(array: [Self::Scalar; 1]) -> Self {
		Self::from(array)
	}

	fn to_array(&self) -> [Self::Scalar; 1] {
		self.0
	}
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color<2> for image::LumaA<Scalar> {
	type Scalar = Scalar;
	type Format = formats::YA;
	type Space = spaces::CieXYZ;

	fn from_array(array: [Self::Scalar; 2]) -> Self {
		Self::from(array)
	}

	fn to_array(&self) -> [Self::Scalar; 2] {
		self.0
	}
}
