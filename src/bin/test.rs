#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "test"
    }

    cli::handle_error(&name, 1, run());
}

fn run() -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let scrn = try!(disp.screen());
    let win = try!(scrn.root());
    let c = try!(win.children());
    let mapped = c.iter().filter(|w| w.mapped());
    for w in mapped {
        println!("{}: x = {}, y = {}", w.id(), w.x(), w.y());
    }
    Ok(())
}
