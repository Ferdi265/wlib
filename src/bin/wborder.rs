#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "change window border",
        opt color: wtools::Color,
            (&["-c", "--color"], "window border color (hexadecimal)"),
        opt size: i32,
            (&["-s", "--size"], "window border size"),
        arg wid: window::ID,
            ("wid", "XServer window id (hexadecimal)")
    }

    cli::handle_error(&name, 1, run(color, size, wid));
}

fn run(color: Option<wtools::Color>, size: Option<i32>, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let mut win = try!(
        disp.window(wid).map_err(|_| "window id does not exist")
    );
    let mut c = window::Changes::new();
    if let Some(color) = color {
        c.border_color(color);
    }
    if let Some(size) = size {
        c.border_width(size);
    }
    try!(win.change(&c));
    Ok(())
}
