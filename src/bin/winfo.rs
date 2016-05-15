#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

#[derive(Copy, Clone)]
enum Attr {
    X,
    Y,
    W,
    H,
    B,
    I,
    M,
    O,
}

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "window information",
        list attrs: Attr,
            (&["-x"], Attr::X, "x coordinate"),
            (&["-y"], Attr::Y, "y coordinate"),
            (&["-w", "--width"], Attr::W, "window width"),
            (&["-h", "--height"], Attr::H, "window height"),
            (&["-b", "--border", "--border-width"], Attr::B, "border size"),
            (&["-i", "--id"], Attr::I, "window id"),
            (&["-m", "--mapped"], Attr::M, "map state"),
            (&["-o", "--ignored", "--override-redirect"], Attr::O, "override_redirect value"),
        arg wid: window::ID,
            ("wid", "window id")
    }
    
    cli::handle_error(&name, 1, run(attrs, wid));
}

fn run(attrs: Vec<Attr>, wid: window::ID) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let win = try!(disp.window(wid));
    for a in attrs {
        match a {
            Attr::X => print!("{} ", win.x()),
            Attr::Y => print!("{} ", win.y()),
            Attr::W => print!("{} ", win.width()),
            Attr::H => print!("{} ", win.height()),
            Attr::B => print!("{} ", win.border_width()),
            Attr::I => print!("{} ", win.id()),
            Attr::M => print!("{} ", win.mapped() as i32),
            Attr::O => print!("{} ", win.ignored() as i32),
        };
    }
    println!("");
    Ok(())
}
