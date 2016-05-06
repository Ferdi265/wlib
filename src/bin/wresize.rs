extern crate getopts;
extern crate wtools;

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
    let r = opts.parse(args);

    let (x, y, w, mode) = wtools::handle_error(&name, 1, parse(r));
    wtools::handle_error(&name, 2, run(x, y, w, mode));
}

fn parse(r: getopts::Result) -> wtools::Result<(i32, i32, u64, Mode)> {
    let matches = try!(r.map_err(|e| match e {
        getopts::Fail::UnrecognizedOption(_) => "unrecognized option",
        getopts::Fail::OptionDuplicated(_) => "option duplicated",
        _ => unreachable!()
    }));

    let mode = try!(if matches.opt_present("r") && matches.opt_present("a") {
        Err("cannot have both -r and -a")
    } else if matches.opt_present("a") {
        Ok(Mode::Absolute)
    } else {
        Ok(Mode::Relative)
    });

    let args = matches.free;

    if args.len() != 3 {
        return Err("missing or extraneous arguments");
    }

    let x = try!(args[0].parse().ok().ok_or("x is not a number"));
    let y = try!(args[1].parse().ok().ok_or("y is not a number"));
    let w = try!(wtools::parse_hex(&args[2]).ok_or("w is not a hexadecimal number"));

    Ok((x, y, w, mode))
}

fn run(x: i32, y: i32, w: u64, mode: Mode) -> wtools::Result<()> {
    let disp = try!(wtools::Display::open());
    let mut win = try!(disp.window(w));
    match mode {
        Mode::Relative => try!(win.resize_relative(x, y)),
        Mode::Absolute => try!(win.resize(x, y))
    }
    Ok(())
}
