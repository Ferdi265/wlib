#![feature(pub_restricted, unique, shared, associated_consts)]

extern crate x11;

pub mod ffi;
pub mod display;
pub mod screen;
pub mod window;
pub mod color;
pub mod shapes;

pub use display::Display;
pub use screen::Screen;
pub use window::Window;
pub use color::Color;
