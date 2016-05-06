extern crate getopts;
#[macro_use]
extern crate wtools;

use std::io::Write;
use std::process;

enum Mode {
    Relative,
    Absolute
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
    
    if matches.free.len() != 3 {
        println_stderr!("{}: missing or extraneous arguments", name);
        process::exit(1);
    }
    
    println!("Mode: {}", match mode {
        Mode::Relative => "rel",
        Mode::Absolute => "abs"
    });
}
