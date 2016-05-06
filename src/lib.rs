#![feature(pub_restricted)]

extern crate x11;

#[macro_use]
mod err;
mod cli;
mod display;
mod screen;
mod window;

pub use err::Result;
pub use cli::args;
pub use cli::number_args;
pub use cli::parse_hex;
pub use cli::handle_error;
pub use display::Display;
pub use screen::Screen;
pub use window::Window;
