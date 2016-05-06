#![feature(pub_restricted)]

extern crate x11;

mod err;
mod cli;
mod display;
mod screen;
mod window;

pub use err::OrErrorStr;
pub use err::handle_error;
pub use cli::args;
pub use cli::number_args;
pub use display::Display;
pub use screen::Screen;
pub use window::Window;
