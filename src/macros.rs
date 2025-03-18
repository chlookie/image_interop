#[macro_export]
macro_rules! declare_color_component {
	($type:ident) => {
		paste::paste! {

			#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
			pub struct $type;
			impl $crate::ColorComponent for $type {}

			pub trait [<$type Component>]<const CHANNELS: $crate::Channels, Scalar> {
				fn component(slice: &[Scalar; CHANNELS]) -> Scalar;
				fn component_mut(slice: &mut [Scalar; CHANNELS]) -> &mut Scalar;
			}

			pub trait [<Has $type>]<Scalar> {
				fn [<$type:snake>](&self) -> Scalar;
			}

			pub trait [<Has $type Mut>]<Scalar>: [<Has $type>]<Scalar> {
				fn [<set_ $type:snake>](&mut self, [<$type:snake>]: Scalar);
			}

			#[allow(dead_code)]
			impl<'a, const CHANNELS: $crate::Channels, C: $crate::Color<CHANNELS>> [<Has $type>]<C::Scalar> for $crate::PixelView<'a, CHANNELS, C>
			where
				C::Format: [<$type Component>]<CHANNELS, C::Scalar>,
			{
				fn [<$type:snake>](&self) -> C::Scalar {
					<C::Format as [<$type Component>]<CHANNELS, C::Scalar>>::component(self.slice)
				}
			}

			#[allow(dead_code)]
			impl<'a, const CHANNELS: $crate::Channels, C: $crate::Color<CHANNELS>> [<Has $type>]<C::Scalar> for $crate::PixelViewMut<'a, CHANNELS, C>
			where
				C::Format: [<$type Component>]<CHANNELS, C::Scalar>,
			{
				fn [<$type:snake>](&self) -> C::Scalar {
					<C::Format as [<$type Component>]<CHANNELS, C::Scalar>>::component(self.slice)
				}
			}

			#[allow(dead_code)]
			impl<'a, const CHANNELS: $crate::Channels, C: $crate::Color<CHANNELS>> [<Has $type Mut>]<C::Scalar> for $crate::PixelViewMut<'a, CHANNELS, C>
			where
				C::Format: [<$type Component>]<CHANNELS, C::Scalar>,
			{
				fn [<set_ $type:snake>](&mut self, [<$type:snake>]: C::Scalar) {
					*<C::Format as [<$type Component>]<CHANNELS, C::Scalar>>::component_mut(self.slice) = [<$type:snake>]
				}
			}

			#[allow(dead_code)]
			impl<const CHANNELS: $crate::Channels, Format, Scalar, Space> [<Has $type>]<Scalar> for $crate::GenericColor<{ CHANNELS }, Format, Scalar, Space>
			where
				Format: [<$type Component>]<CHANNELS, Scalar>,
			{
				fn [<$type:snake>](&self) -> Scalar {
					<Format as [<$type Component>]<CHANNELS, Scalar>>::component(&self.color)
				}
			}

			#[allow(dead_code)]
			impl<const CHANNELS: $crate::Channels, Format, Scalar, Space> [<Has $type Mut>]<Scalar> for $crate::GenericColor<{ CHANNELS }, Format, Scalar, Space>
			where
				Format: [<$type Component>]<CHANNELS, Scalar>,
			{
				fn [<set_ $type:snake>](&mut self, [<$type:snake>]: Scalar) {
					*<Format as [<$type Component>]<CHANNELS, Scalar>>::component_mut(&mut self.color) = [<$type:snake>]
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
	($name:ident: $($channels:ty),+) => {paste::paste!{
		#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
		pub struct $name;
		const [<$name _CHANNELS>]: $crate::Channels = declare_color_format!(@count_channels $($channels,)*);

		impl $crate::ColorFormat<{ [<$name _CHANNELS>] }> for $name {}

		impl<From, Scalar> $crate::ConvertFormatFrom<{ [<$name _CHANNELS>] }, From, Scalar> for $name
		where
			From: $crate::ColorFormat<{ [<$name _CHANNELS>] }> $(+ [<$channels Component>]<{ [<$name _CHANNELS>] }, Scalar>)+,
		{
			fn convert_array(array: [Scalar; [<$name _CHANNELS>]]) -> [Scalar; [<$name _CHANNELS>]] {
				[$(
					<From as [<$channels Component>]<[<$name _CHANNELS>], Scalar>>::component(&array),
				)*]
			}
		}

		declare_color_format!(@iter_channels 0, $name: $($channels,)+);
	}};

	(@iter_channels $index:expr, $name:ident: ) => {};

	(@iter_channels $index:expr, $name:ident: $channel:ty, $($channels:ty,)*) => {paste::paste!{
		#[allow(unused_imports)]
		use $crate::components::*;

		impl<Scalar: $crate::ScalarPrimitive> [<$channel Component>]<{[<$name _CHANNELS>]}, Scalar> for $name {
			fn component(array: &[Scalar; [<$name _CHANNELS>]]) -> Scalar {
				array[$index]
			}

			fn component_mut(array: &mut [Scalar; [<$name _CHANNELS>]]) -> &mut Scalar {
				&mut array[$index]
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
