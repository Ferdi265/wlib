extern crate getopts;
#[macro_use]
extern crate wtools;

use std::io::Write;
use std::process;

enum Mode {
    Relative,
    Absolute
}

fn parse(args: Vec<String>) -> wtools::OrError<(i32, i32, u64)> {
    if args.len() != 3 {
        return Err("missing or extraneous arguments".to_string());
    }

    let x = try!(args[0].parse().map_err(|_| "x is not a number".to_string()));
    let y = try!(args[1].parse().map_err(|_| "y is not a number".to_string()));
    let w = try!(wtools::parse_hex(&args[2]).ok_or("w is not a hexadecimal number".to_string()));

    Ok((x, y, w))
}

fn run(x: i32, y: i32, w: u64, mode: Mode) -> wtools::OrError<()> {
    let disp = try!(wtools::Display::open());
    let mut win = try!(disp.window(w));
    match mode {
        Mode::Relative => try!(win.position_relative(x, y)),
        Mode::Absolute => try!(win.position(x, y))
    }
    Ok(())
}

fn main() {
    let (name, args) = wtools::number_args();
    
    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.optflag("r", "relative", "");
    opts.optflag("a", "absolute", "");
    
    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => {
            println_stderr!("{}: {}", name, f.to_string());
            process::exit(1);
        }
    };
    
    let mode = if matches.opt_present("r") && matches.opt_present("a") {
        println_stderr!("{}: cannot have both -r and -a", name);
        process::exit(1);
    } else if matches.opt_present("a") {
        Mode::Absolute
    } else {
        Mode::Relative
    };

    let (x, y, w) = wtools::handle_error(&name, 1, parse(matches.free));
    wtools::handle_error(&name, 2, run(x, y, w, mode));
}
