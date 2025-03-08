use std::marker::PhantomData;

use crate::{Channels, ColorFormat, Pixel, PixelToComponents, ScalarPrimitive};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub trait TupleLength<T> {
	type Tuple;
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Length<const N: usize>;

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
pub struct GenericColor<const CHANNELS: Channels, Format, Scalar> {
	pub color: [Scalar; CHANNELS],
	_format: PhantomData<Format>,
}

impl<const CHANNELS: Channels, Format, Scalar> Default for GenericColor<{ CHANNELS }, Format, Scalar>
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

impl<const CHANNELS: Channels, Format, Scalar> Pixel for GenericColor<{ CHANNELS }, Format, Scalar>
where
	Scalar: ScalarPrimitive,
	Format: Copy + ColorFormat<Scalar>,
{
	type Scalar = Scalar;
	type Format = Format;
}

impl<const CHANNELS: Channels, Format, Scalar> PixelToComponents for GenericColor<{ CHANNELS }, Format, Scalar>
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
