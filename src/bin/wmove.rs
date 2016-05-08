#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;

#[derive(Copy, Clone)]
enum Mode {
    Relative,
    Absolute
}

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "move a window on the XServer",
        opt mode: Mode = Mode::Relative,
            (&["-r", "--relative"], Mode::Relative, "move window relatively"),
            (&["-a", "--absolute"], Mode::Absolute, "move window absolutely"),
        arg x: i32 = 0,
            ("x", "x coordinate"),
        arg y: i32 = 0,
            ("y", "y coordinate"),
        arg win: wtools::WindowID = 0x0.into(),
            ("win", "XServer window id")
    }

    cli::handle_error(&name, 2, run(mode, x, y, win));
}

fn run(mode: Mode, x: i32, y: i32, w: wtools::WindowID) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let mut win = try!(
        disp.window(w).map_err(|_| "window id does not exist")
    );
    match mode {
        Mode::Relative => try!(win.position_relative(x, y)),
        Mode::Absolute => try!(win.position(x, y))
    }
    Ok(())
}
