use std::{fmt::Debug, marker::PhantomData};

use crate::{Channels, Color, ColorComponents, ColorFormat, ScalarPrimitive};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub mod components {
	use crate::declare_color_component;

	declare_color_component!(Alpha);
	declare_color_component!(Red);
	declare_color_component!(Green);
	declare_color_component!(Blue);
	declare_color_component!(Hue);
	declare_color_component!(Whiteness);
	declare_color_component!(Blackness);
	declare_color_component!(Saturation);
	declare_color_component!(Value);
	declare_color_component!(Lightness);
	declare_color_component!(A);
	declare_color_component!(B);
	declare_color_component!(Chroma);
	declare_color_component!(X);
	declare_color_component!(Y);
	declare_color_component!(Z);
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// Const generic CHANNELS required because we don't have const_generic_expressions yet
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

impl<const CHANNELS: Channels, Format, Scalar> Color for GenericColor<{ CHANNELS }, Format, Scalar>
where
	Scalar: ScalarPrimitive,
	Format: Copy + ColorFormat<Scalar>,
{
	type Scalar = Scalar;
	type Format = Format;
}

macro_rules! impl_color_components_for_generic_color {
	($channels:expr, $tuple:ty) => {
		impl<Format, Scalar> ColorComponents for GenericColor<{ $channels }, Format, Scalar>
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

#[rustfmt::skip] impl_color_components_for_generic_color!(1,  (Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(2,  (Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(3,  (Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(4,  (Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(5,  (Scalar, Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(6,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(7,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(8,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(9,  (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(10, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(11, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,));
#[rustfmt::skip] impl_color_components_for_generic_color!(12, (Scalar, Scalar, Scalar, Scalar, Scalar, Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,Scalar,));
