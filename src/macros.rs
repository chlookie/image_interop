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
			impl<'a, P: $crate::Pixel> [<Has $type>]<P::Scalar> for $crate::PixelView<'a, P>
			where
				P::Format: [<$type Component>]<P::Scalar>,
			{
				fn [<$type:snake>](&self) -> P::Scalar {
					<P::Format as [<$type Component>]<P::Scalar>>::component(self.slice)
				}
			}

			#[allow(dead_code)]
			impl<'a, P: $crate::Pixel> [<Has $type>]<P::Scalar> for $crate::PixelViewMut<'a, P>
			where
				P::Format: [<$type Component>]<P::Scalar>,
			{
				fn [<$type:snake>](&self) -> P::Scalar {
					<P::Format as [<$type Component>]<P::Scalar>>::component(self.slice)
				}
			}

			#[allow(dead_code)]
			impl<'a, P: $crate::Pixel> [<Has $type Mut>]<P::Scalar> for $crate::PixelViewMut<'a, P>
			where
				P::Format: [<$type Component>]<P::Scalar>,
			{
				fn [<set_ $type:snake>](&mut self, [<$type:snake>]: P::Scalar) {
					*<P::Format as [<$type Component>]<P::Scalar>>::component_mut(self.slice) = [<$type:snake>]
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
	($alias:ident: $($channels:ty),+) => {

		#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
		pub struct $alias;

		impl<Scalar: $crate::ScalarPrimitive> $crate::ColorFormat<Scalar> for $alias {
			const CHANNELS: $crate::Channels = declare_color_format!(@count_channels $($channels,)*);
		}

		declare_color_format!(@iter_channels 0, $alias: $($channels,)+);
	};

	(@iter_channels $index:expr, $alias:ident: ) => {};

	(@iter_channels $index:expr, $alias:ident: $channel:ty, $($channels:ty,)*) => {paste::paste!{
		#[allow(unused_imports)]
		use $crate::components::*;

		impl<Scalar: $crate::ScalarPrimitive> [<$channel Component>]<Scalar> for $alias {
			fn component(slice: &[Scalar]) -> Scalar {
				slice[$index]
			}

			fn component_mut(slice: &mut [Scalar]) -> &mut Scalar {
				&mut slice[$index]
			}
		}

		declare_color_format!(@iter_channels ($index+1), $alias: $($channels,)*);
	}};

	(@count_channels $channel:ty, $($channels:ty,)*) => {1 + declare_color_format!(@count_channels $($channels,)*)};
	(@count_channels ) => {0};
}
