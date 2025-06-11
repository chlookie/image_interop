use crate::{Color, ConvertImage, GenericImage, InterleavedImageLayout, ScalarPrimitive, formats, spaces};

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

impl<S, B, L> ConvertImage<GenericImage<3, image::Rgb<S>, L, B>> for image::ImageBuffer<image::Rgb<S>, Vec<S>>
where
	image::Rgb<S>: image::Pixel,
	S: image::Primitive + ScalarPrimitive,
	L: InterleavedImageLayout,
{
	fn convert_image(self) -> GenericImage<3, image::Rgb<S>, L, B> {
		GenericImage::from_buffer(self.into_vec(), layout)
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
