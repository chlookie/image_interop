use std::marker::PhantomData;

use crate::{Channels, ColorFormat, Pixel, PixelToComponents, ScalarPrimitive};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Const generic CHANNELS required because we don't have const_generic_expressions yet
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color<const CHANNELS: Channels, Format, Scalar> {
	pub color: [Scalar; CHANNELS],
	_format: PhantomData<Format>,
}

impl<const CHANNELS: Channels, Format, Scalar> Default for Color<{ CHANNELS }, Format, Scalar>
where
	Scalar: Default + Copy,
{
	fn default() -> Self {
		Self {
			color: [Default::default(); CHANNELS],
			_format: Default::default(),
		}
	}
}

impl<const CHANNELS: Channels, Format, Scalar> Pixel for Color<{ CHANNELS }, Format, Scalar>
where
	Scalar: ScalarPrimitive,
	Format: Copy + ColorFormat<Scalar>,
{
	type Scalar = Scalar;
	type Format = Format;
}

macro_rules! impl_pixel_to_components {
	($channels:expr, $tuple:ty) => {
		impl<Format, Scalar> PixelToComponents for Color<{ $channels }, Format, Scalar>
		where
			Scalar: ScalarPrimitive,
			Format: Copy + ColorFormat<Scalar>,
		{
			type Tuple = $tuple;
			type Array = [Scalar; $channels];

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
				Self {
					color: array,
					_format: PhantomData,
				}
			}

			fn to_array(&self) -> Self::Array {
				self.color
			}
		}
	};
}

#[rustfmt::skip] impl_pixel_to_components!(1,  (Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(2,  (Scalar, Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(3,  (Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(4,  (Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(5,  (Scalar, Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(6,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(7,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(8,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(9,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(10, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(11, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_pixel_to_components!(12, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,));
