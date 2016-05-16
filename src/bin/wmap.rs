#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

#[derive(Copy, Clone)]
enum Mode {
    Map,
    Unmap
}

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "map or unmap window",
        flag mode: Mode = Mode::Map,
            (&["-m", "--map"], Mode::Map, "map window (default)"),
            (&["-u", "--unmap"], Mode::Unmap, "unmap window"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli::handle_error(&name, 1, run(mode, wid));
}

fn run(mode: Mode, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let mut win = try!(
        disp.window(wid).map_err(|_| "window does not exist")
    );
    match mode {
        Mode::Map => try!(win.map()),
        Mode::Unmap => try!(win.unmap())
    }
    Ok(())
}
