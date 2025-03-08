use std::{
	marker::Send,
	ops::{Deref, DerefMut},
};

use rayon::{
	iter::{IndexedParallelIterator, ParallelIterator},
	slice::{ParallelSlice, ParallelSliceMut},
};

use crate::{PixelView, PixelViewMut, image::Image, pixel::Pixel};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub trait ImageParallelIter {
	/// The type of pixel.
	type Pixel: Pixel;

	/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
	fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<Self::Pixel>>;

	/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
	fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<Self::Pixel>)>;
}

pub trait ImageParallelIterMut {
	/// The type of pixel.
	type Pixel: Pixel;

	/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
	fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<Self::Pixel>>;

	/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
	fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<Self::Pixel>)>;
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl<P, Buffer> ImageParallelIter for Image<P, Buffer>
where
	P: Pixel + Sync,
	P::Scalar: Sync,
	P::Format: Sync + Send,
	Buffer: Deref<Target = [P::Scalar]> + Sync,
{
	type Pixel = P;

	/// Returns a parallel iterator over the pixels of the image, usable with `rayon`.
	fn par_pixels(&self) -> impl ParallelIterator<Item = PixelView<P>> {
		let (outer_stride, outer_size, inner_stride, inner_size) = self.iter_strides();

		self.buffer
			.par_chunks_exact(outer_stride)
			.flat_map(move |padded_chunk| {
				let chunk = &padded_chunk[..outer_size];

				chunk.par_chunks_exact(inner_stride).map(move |padded_pixel| {
					let slice = &padded_pixel[..inner_size];
					P::as_view_unchecked(slice)
				})
			})
	}

	/// Returns a parallel iterator over the pixels of the image and their respective coordinates, usable with `rayon`.
	fn par_enumerate_pixels(&self) -> impl ParallelIterator<Item = (u32, u32, PixelView<P>)> {
		let (outer_stride, outer_size, inner_stride, inner_size) = self.iter_strides();
		let is_row_major = self.layout.is_row_major();

		self.buffer
			.par_chunks_exact(outer_stride)
			.enumerate()
			.flat_map(move |(outer_index, padded_chunk)| {
				let chunk = &padded_chunk[..outer_size];

				chunk
					.par_chunks_exact(inner_stride)
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

impl<P, Buffer> ImageParallelIterMut for Image<P, Buffer>
where
	P: Pixel + Send + Sync,
	P::Scalar: Send + Sync,
	P::Format: Send + Sync,
	Buffer: DerefMut<Target = [P::Scalar]> + Send + Sync,
{
	type Pixel = P;

	/// Returns a parallel iterator over the mutable pixels of the image, usable with `rayon`.
	fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = PixelViewMut<P>> {
		let (outer_stride, outer_size, inner_stride, inner_size) = self.iter_strides();

		self.buffer
			.par_chunks_exact_mut(outer_stride)
			.flat_map(move |padded_chunk| {
				let chunk = &mut padded_chunk[..outer_size];

				chunk.par_chunks_exact_mut(inner_stride).map(move |padded_pixel| {
					let slice = &mut padded_pixel[..inner_size];
					P::as_view_mut_unchecked(slice)
				})
			})
	}

	/// Returns a parallel iterator over the mutable pixels of the image and their respective coordinates, usable with `rayon`.
	fn par_enumerate_pixels_mut(&mut self) -> impl ParallelIterator<Item = (u32, u32, PixelViewMut<P>)> {
		let (outer_stride, outer_size, inner_stride, inner_size) = self.iter_strides();
		let is_row_major = self.layout.is_row_major();

		self.buffer
			.par_chunks_exact_mut(outer_stride)
			.enumerate()
			.flat_map(move |(outer_index, padded_chunk)| {
				let chunk = &mut padded_chunk[..outer_size];

				chunk
					.par_chunks_exact_mut(inner_stride)
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
