# Image_interop
A work-in-progress crate for interoperability between image formats and color spaces.

The crate is designed to not require support on the receiving crates' end, and instead uses local traits to extend functionality where needed.

Currently offers support for the `image` and `bevy_color` crates as a test, but the crate is designed to be easily extendable.
Plans include support for `zune-image`, `color`, and more!

