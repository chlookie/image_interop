use std::marker::PhantomData;

use crate::{Channels, ColorFormat, Pixel, PixelToComponents, ScalarPrimitive};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

trait TupleLength<T> {
	type Tuple;
}

struct Length<const N: usize>;

#[rustfmt::skip] impl<T> TupleLength<T> for Length<1>  {type Tuple = (T,);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<2>  {type Tuple = (T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<3>  {type Tuple = (T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<4>  {type Tuple = (T, T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<5>  {type Tuple = (T, T, T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<6>  {type Tuple = (T, T, T, T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<7>  {type Tuple = (T, T, T, T, T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<8>  {type Tuple = (T, T, T, T, T, T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<9>  {type Tuple = (T, T, T, T, T, T, T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<10> {type Tuple = (T, T, T, T, T, T, T, T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<11> {type Tuple = (T, T, T, T, T, T, T, T, T, T, T);}
#[rustfmt::skip] impl<T> TupleLength<T> for Length<12> {type Tuple = (T, T, T, T, T, T, T, T, T, T, T, T);}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct GenericPixel<Format, Scalar, const CHANNELS: Channels> {
	pub color: [Scalar; CHANNELS],
	_format: PhantomData<Format>,
}

impl<Format, Scalar, const CHANNELS: Channels> Default for GenericPixel<Format, Scalar, { CHANNELS }>
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

impl<Format, Scalar, const CHANNELS: Channels> Pixel for GenericPixel<Format, Scalar, { CHANNELS }>
where
	Scalar: ScalarPrimitive,
	Format: Copy + ColorFormat<Scalar>,
{
	type Scalar = Scalar;
	type Format = Format;
}

impl<Format, Scalar, const CHANNELS: Channels> PixelToComponents for GenericPixel<Format, Scalar, { CHANNELS }>
where
	Scalar: ScalarPrimitive,
	Format: Copy + ColorFormat<Scalar>,
	Length<CHANNELS>: TupleLength<Scalar>,
	[Scalar; CHANNELS]: From<<Length<CHANNELS> as TupleLength<Scalar>>::Tuple>,
	<Length<CHANNELS> as TupleLength<Scalar>>::Tuple: From<[Scalar; CHANNELS]>,
{
	type Tuple = <Length<CHANNELS> as TupleLength<Scalar>>::Tuple;
	type Array = [Scalar; CHANNELS];

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
