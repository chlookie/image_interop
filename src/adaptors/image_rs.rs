use crate::{Color, ColorComponents, ScalarPrimitive, formats, spaces};

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

impl<S: image::Primitive + ScalarPrimitive> ColorComponents for image::Rgb<S> {
	type Tuple = (S, S, S);
	type Array = [S; 3];

	fn from_slice_unchecked(slice: &[Self::Scalar]) -> Self {
		Self::from_array(slice.try_into().expect("Slice must have exactly 3 elements"))
	}

	fn from_tuple(tuple: Self::Tuple) -> Self {
		Self::from_array([tuple.0, tuple.1, tuple.2])
	}

	fn to_tuple(&self) -> Self::Tuple {
		(self.0[0], self.0[1], self.0[2])
	}

	fn from_array(array: Self::Array) -> Self {
		Self::from(array)
	}

	fn to_array(&self) -> Self::Array {
		self.0
	}
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color for image::Rgba<Scalar> {
	type Scalar = Scalar;
	type Format = formats::RGBA;
	type Space = spaces::SRGB;
}

impl<S: image::Primitive + ScalarPrimitive> ColorComponents for image::Rgba<S> {
	type Tuple = (S, S, S, S);
	type Array = [S; 4];

	fn from_slice_unchecked(slice: &[Self::Scalar]) -> Self {
		Self::from_array(slice.try_into().expect("Slice must have exactly 4 elements"))
	}

	fn from_tuple(tuple: Self::Tuple) -> Self {
		Self::from_array([tuple.0, tuple.1, tuple.2, tuple.3])
	}

	fn to_tuple(&self) -> Self::Tuple {
		(self.0[0], self.0[1], self.0[2], self.0[3])
	}

	fn from_array(array: Self::Array) -> Self {
		Self::from(array)
	}

	fn to_array(&self) -> Self::Array {
		self.0
	}
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color for image::Luma<Scalar> {
	type Scalar = Scalar;
	type Format = formats::Y;
	type Space = spaces::CieXYZD65;
}

impl<S: image::Primitive + ScalarPrimitive> ColorComponents for image::Luma<S> {
	type Tuple = (S,);
	type Array = [S; 1];

	fn from_slice_unchecked(slice: &[Self::Scalar]) -> Self {
		Self::from_array(slice.try_into().expect("Slice must have exactly 1 element"))
	}

	fn from_tuple(tuple: Self::Tuple) -> Self {
		Self::from_array([tuple.0])
	}

	fn to_tuple(&self) -> Self::Tuple {
		(self.0[0],)
	}

	fn from_array(array: Self::Array) -> Self {
		Self::from(array)
	}

	fn to_array(&self) -> Self::Array {
		self.0
	}
}

impl<Scalar: image::Primitive + ScalarPrimitive> Color for image::LumaA<Scalar> {
	type Scalar = Scalar;
	type Format = formats::YA;
	type Space = spaces::CieXYZD65;
}

impl<S: image::Primitive + ScalarPrimitive> ColorComponents for image::LumaA<S> {
	type Tuple = (S, S);
	type Array = [S; 2];

	fn from_slice_unchecked(slice: &[Self::Scalar]) -> Self {
		Self::from_array(slice.try_into().expect("Slice must have exactly 2 elements"))
	}

	fn from_tuple(tuple: Self::Tuple) -> Self {
		Self::from_array([tuple.0, tuple.1])
	}

	fn to_tuple(&self) -> Self::Tuple {
		(self.0[0], self.0[1])
	}

	fn from_array(array: Self::Array) -> Self {
		Self::from(array)
	}

	fn to_array(&self) -> Self::Array {
		self.0
	}
}
