#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "returns the root window"
    }

    cli::handle_error(&name, 1, run());
}

fn run() -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let scrn = try!(disp.screen());
    let win = try!(scrn.root());
    println!("{}", win.id());
    Ok(())
}
