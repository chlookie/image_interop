#[macro_export]
macro_rules! declare_color_component {
	($type:ident) => {
		paste::paste! {

			#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
			pub struct $type;
			impl $crate::ColorComponent for $type {}

			pub trait [<$type Component>]<Scalar> {
				fn component(slice: &[Scalar]) -> Scalar;
				fn component_mut(slice: &mut [Scalar]) -> &mut Scalar;
			}

			pub trait [<Has $type>]<Scalar> {
				fn [<$type:snake>](&self) -> Scalar;
			}

			pub trait [<Has $type Mut>]<Scalar>: [<Has $type>]<Scalar> {
				fn [<set_ $type:snake>](&mut self, [<$type:snake>]: Scalar);
			}

			#[allow(dead_code)]
			impl<'a, C: $crate::Color> [<Has $type>]<C::Scalar> for $crate::PixelView<'a, C>
			where
				C::Format: [<$type Component>]<C::Scalar>,
			{
				fn [<$type:snake>](&self) -> C::Scalar {
					<C::Format as [<$type Component>]<C::Scalar>>::component(self.slice)
				}
			}

			#[allow(dead_code)]
			impl<'a, C: $crate::Color> [<Has $type>]<C::Scalar> for $crate::PixelViewMut<'a, C>
			where
				C::Format: [<$type Component>]<C::Scalar>,
			{
				fn [<$type:snake>](&self) -> C::Scalar {
					<C::Format as [<$type Component>]<C::Scalar>>::component(self.slice)
				}
			}

			#[allow(dead_code)]
			impl<'a, C: $crate::Color> [<Has $type Mut>]<C::Scalar> for $crate::PixelViewMut<'a, C>
			where
				C::Format: [<$type Component>]<C::Scalar>,
			{
				fn [<set_ $type:snake>](&mut self, [<$type:snake>]: C::Scalar) {
					*<C::Format as [<$type Component>]<C::Scalar>>::component_mut(self.slice) = [<$type:snake>]
				}
			}
		}
	};
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[macro_export]
macro_rules! declare_color_format {
	($name:ident: $($channels:ty),+) => {

		#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
		pub struct $name;

		impl<Scalar: $crate::ScalarPrimitive> $crate::ColorFormat<Scalar> for $name {
			const CHANNELS: $crate::Channels = declare_color_format!(@count_channels $($channels,)*);
		}

		declare_color_format!(@iter_channels 0, $name: $($channels,)+);
	};

	(@iter_channels $index:expr, $name:ident: ) => {};

	(@iter_channels $index:expr, $name:ident: $channel:ty, $($channels:ty,)*) => {paste::paste!{
		#[allow(unused_imports)]
		use $crate::components::*;

		impl<Scalar: $crate::ScalarPrimitive> [<$channel Component>]<Scalar> for $name {
			fn component(slice: &[Scalar]) -> Scalar {
				slice[$index]
			}

			fn component_mut(slice: &mut [Scalar]) -> &mut Scalar {
				&mut slice[$index]
			}
		}

		declare_color_format!(@iter_channels ($index+1), $name: $($channels,)*);
	}};

	(@count_channels $channel:ty, $($channels:ty,)*) => {1 + declare_color_format!(@count_channels $($channels,)*)};
	(@count_channels ) => {0};
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[macro_export]
macro_rules! declare_color_space {
	($name:ident) => {
		declare_color_space!($name, "");
	};

	($name:ident, $doc:expr) => {
		#[doc = $doc]
		#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
		pub struct $name;
		impl $crate::ColorSpace for $name {}
	};
}
