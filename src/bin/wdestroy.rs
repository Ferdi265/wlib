#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

#[derive(Copy, Clone)]
enum Mode {
    Destroy,
    Kill
}

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "destroy window",
        flag mode: Mode = Mode::Destroy,
            (&["-k", "--kill"], Mode::Kill, "kills the client controlling the window"),
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
        Mode::Destroy => try!(win.destroy()),
        Mode::Kill => try!(win.kill())
    }
    Ok(())
}
