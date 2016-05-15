#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "change window border",
        arg color: window::Color = window::Color::from_i32(0),
            ("color", "window border color (hexadecimal)"),
        arg size: i32 = 0,
            ("size", "window border size"),
        arg wid: window::ID = 0.into(),
            ("wid", "XServer window id (hexadecimal)")
    }

    cli::handle_error(&name, 1, run(color, size, wid));
}

fn run(color: window::Color, size: i32, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let mut win = try!(
        disp.window(wid).map_err(|_| "window id does not exist")
    );
    let mut c = window::Changes::new();
    c.border_color(color);
    c.border_width(size);
    try!(win.change(&c));
    Ok(())
}
