#![feature(pub_restricted)]

extern crate x11;

mod err;
mod display;
mod screen;

pub use err::OrErrorStr;
pub use err::handle_error;
pub use display::Display;
pub use screen::Screen;
