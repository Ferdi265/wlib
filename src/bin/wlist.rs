#[macro_use]
extern crate wtools;

use std::env;
use wtools::cli;
use wtools::window;

fn main() {
    let name = cli::name(&mut env::args());

    parse_args!{
        description: "list windows",
        flag u: bool = false,
            (&["-u", "--unmapped"], true, "list unmapped windows"),
        flag o: bool = false,
            (&["-o", "--override"], true, "list windows with override_redirect"),
        flag a: bool = false,
            (&["-a", "--all"], true, "list all windows, implies -u and -o"),
        optarg wid: window::ID,
            ("wid", "window id to list children from, defaults to the root window")
    }
    
    let mut u = u;
    let mut o = o;

    if a {
        u = true;
        o = true;
    }

    cli::handle_error(&name, 1, run(u, o, wid));
}

fn run(u: bool, o: bool, wid: Option<window::ID>) -> Result<(), &'static str> {
    let disp = try!(wtools::Display::open());
    let win = match wid {
        Some(w) => try!(disp.window(w)),
        None => {
            let scrn = try!(disp.screen());
            try!(scrn.root())
        }
    };
    let c = try!(win.children());
    let iter = c.iter()
        .filter(|w| u || w.mapped())
        .filter(|w| o || !w.ignored());
    for w in iter {
        println!("{}", w.id());
    }
    Ok(())
}
