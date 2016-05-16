#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "focus window",
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli::handle_error(&name, 1, run(wid));
}

fn run(wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let win = try!(
        disp.window(wid).map_err(|_| "window does not exist")
    );
    try!(win.focus());
    Ok(())
}
