//! TODO: Add crate description
//!
//!
//!
//!

#![deny(warnings)]
#![deny(missing_docs)]

mod canvas;
mod color;
mod path_builder;
/**
 * A collection of backend renderers
 *
 * One of the main benefits of using `barium` over another, lower-level library is the ability to easily export to a variety of formats.
 * This is accomplished by using differant renderers. It is also incredibly easy to implement your own renderers.
 *
 * This module contains several basic renderers for everyday use. They also serve as referance if you want to implement your own renderer.
 */
pub mod renderers;

pub use canvas::{Canvas, LineEnd, Renderer, Shape, Stroke};
pub use color::Color;
pub use glam::{Mat2, UVec2, Vec2};
pub use image::RgbaImage;
pub use path_builder::PathBuilder;
