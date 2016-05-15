#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

#[derive(Copy, Clone)]
enum Mode {
    Relative,
    Absolute
}

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "resize window",
        flg mode: Mode = Mode::Relative,
            (&["-r", "--relative"], Mode::Relative, "resize relatively"),
            (&["-a", "--absolute"], Mode::Absolute, "resize absolutely"),
        arg x: i32,
            ("x", "x coordinate"),
        arg y: i32,
            ("y", "y coordinate"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli::handle_error(&name, 1, run(mode, x, y, wid));
}

fn run(mode: Mode, x: i32, y: i32, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let mut win = try!(
        disp.window(wid).map_err(|_| "window does not exist")
    );
    match mode {
        Mode::Relative => try!(win.resize_relative(x, y)),
        Mode::Absolute => try!(win.resize_absolute(x, y))
    }
    Ok(())
}
