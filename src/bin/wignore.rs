#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "ignore window",
        flag mode: bool = true,
            (&["-r", "--reset"], false, "sets override_redirect to false"),
            (&["-s", "--set"], true, "sets override_redirect to true (default)"),
        arg wid: window::ID,
            ("wid", "window id")
    }

    cli::handle_error(&name, 1, run(mode, wid));
}

fn run(mode: bool, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let mut win = try!(
        disp.window(wid).map_err(|_| "window does not exist")
    );
    try!(win.ignore(mode));
    Ok(())
}
