use std::ops::{Deref, DerefMut};

use crate::{Image, Pixel, PixelView, PixelViewMut};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<Buffer, P: Pixel> Image<P, Buffer> {
	pub(crate) fn iter_strides(&self) -> (usize, usize, usize, usize) {
		if self.layout.is_row_major() {
			// Iterate over rows first and then columns
			(
				self.layout.stride_y as usize,
				self.layout.width as usize,
				self.layout.stride_x as usize,
				self.channels() as usize,
			)
		} else {
			// Iterate over columns first and then rows
			(
				self.layout.stride_x as usize,
				self.layout.height as usize,
				self.layout.stride_y as usize,
				self.channels() as usize,
			)
		}
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub trait ImageIter {
	/// The type of pixel.
	type Pixel: Pixel;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<Self::Pixel>>;

	/// Returns an iterator over the pixels of the image and their respective coordinates.
	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<Self::Pixel>)>;
}

pub trait ImageIterMut {
	/// The type of pixel.
	type Pixel: Pixel;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<Self::Pixel>>;

	/// Returns an iterator over the pixels of the image and their respective coordinates.
	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<Self::Pixel>)>;
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<P, Buffer> ImageIter for Image<P, Buffer>
where
	P: Pixel,
	Buffer: Deref<Target = [P::Scalar]>,
{
	type Pixel = P;

	/// Returns an iterator over the pixels of the image.
	fn iter_pixels(&self) -> impl Iterator<Item = PixelView<P>> {
		let (outer_stride, outer_size, inner_stride, inner_size) = self.iter_strides();

		self.buffer.chunks_exact(outer_stride).flat_map(move |padded_chunk| {
			let chunk = &padded_chunk[..outer_size];

			chunk.chunks_exact(inner_stride).map(move |padded_pixel| {
				let slice = &padded_pixel[..inner_size];
				P::as_view_unchecked(slice)
			})
		})
	}

	/// Returns an iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
	fn enumerate_pixels(&self) -> impl Iterator<Item = (u32, u32, PixelView<P>)> {
		let (outer_stride, outer_size, inner_stride, inner_size) = self.iter_strides();
		let is_row_major = self.layout.is_row_major();

		self.buffer
			.chunks_exact(outer_stride)
			.enumerate()
			.flat_map(move |(outer_index, padded_chunk)| {
				let chunk = &padded_chunk[..outer_size];

				chunk
					.chunks_exact(inner_stride)
					.enumerate()
					.map(move |(inner_index, padded_pixel)| {
						let (x, y) = if is_row_major {
							(inner_index, outer_index)
						} else {
							(outer_index, inner_index)
						};
						let slice = &padded_pixel[..inner_size];
						let pixel = P::as_view_unchecked(slice);

						(x as u32, y as u32, pixel)
					})
			})
	}
}

impl<P, Buffer> ImageIterMut for Image<P, Buffer>
where
	P: Pixel,
	Buffer: DerefMut<Target = [P::Scalar]>,
{
	type Pixel = P;

	/// Returns an iterator over the mutable pixels of the image, usable with `rayon`.
	fn iter_pixels_mut(&mut self) -> impl Iterator<Item = PixelViewMut<P>> {
		let (outer_stride, outer_size, inner_stride, inner_size) = self.iter_strides();

		self.buffer
			.chunks_exact_mut(outer_stride)
			.flat_map(move |padded_chunk| {
				let chunk = &mut padded_chunk[..outer_size];

				chunk.chunks_exact_mut(inner_stride).map(move |padded_pixel| {
					let slice = &mut padded_pixel[..inner_size];
					P::as_view_mut_unchecked(slice)
				})
			})
	}

	/// Returns an iterator over the mutable pixels of the image and their respective coordinates, usable with `rayon`.
	fn enumerate_pixels_mut(&mut self) -> impl Iterator<Item = (u32, u32, PixelViewMut<P>)> {
		let (outer_stride, outer_size, inner_stride, inner_size) = self.iter_strides();
		let is_row_major = self.layout.is_row_major();

		self.buffer
			.chunks_exact_mut(outer_stride)
			.enumerate()
			.flat_map(move |(outer_index, padded_chunk)| {
				let chunk = &mut padded_chunk[..outer_size];

				chunk
					.chunks_exact_mut(inner_stride)
					.enumerate()
					.map(move |(inner_index, padded_pixel)| {
						let (x, y) = if is_row_major {
							(inner_index, outer_index)
						} else {
							(outer_index, inner_index)
						};
						let slice = &mut padded_pixel[..inner_size];
						let pixel = P::as_view_mut_unchecked(slice);

						(x as u32, y as u32, pixel)
					})
			})
	}
}
