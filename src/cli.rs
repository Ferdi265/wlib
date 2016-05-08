use std::process;
use std::path;
use std::env;
use std::fmt;

/// A macro like `println!` that writes to `stderr` instead
#[macro_export]
macro_rules! println_stderr {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    }}
}

/// A macro that uses the `argparse` crate to parse cmdline arguments
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate wtools;
/// # use wtools::WindowID;
/// # fn main() {
/// # std::process::exit(0);
/// #[derive(Copy, Clone, Debug)]
/// enum Mode {
///     Relative,
///     Absolute
/// }
///
/// parse_args!{
///     description: "a cli utility",
///     opt mode: Mode = Mode::Relative,
///         (&["-r", "--relative"], Mode::Relative, "do sth relatively"),
///         (&["-r", "--absolute"], Mode::Absolute, "do sth absolutely"),
///     arg x: i32 = 0,
///         ("x", "x coordinate"),
///     arg y: i32 = 0,
///         ("y", "y coordinate"),
///     arg wid: WindowID = 0.into(),
///         ("wid", "XServer window id (hexadecimal)")
/// }
///
/// let id: u64 = wid.into();
///
/// println!("mode: {:?}, x: {}, y: {}, wid: {}", mode, x, y, id);
/// # }
/// ```
#[macro_export]
macro_rules! parse_args {
    {
        description : $desc:expr ,
        $(
            opt $opt:ident : $otype:ty = $odefault:expr , $(
                ( $onames:expr, $ovalue:expr , $ohelp:expr )
            ),*
        ),* ,
        $(
            arg $arg:ident : $atype:ty = $adefault:expr ,
            ( $aname: expr , $ahelp:expr )
        ),*
    } => {
        $(
            let mut $opt: $otype = $odefault;
        )*
        $(
            let mut $arg: $atype = $adefault;
        )*
        {
            extern crate argparse;
            let mut ap = argparse::ArgumentParser::new();
            ap.set_description($desc);
            ap.stop_on_first_argument(true);
            $(
                ap.refer(&mut $opt) $(
                    .add_option($onames, argparse::StoreConst($ovalue), $ohelp)
                )*;
            )*
            $(
                ap.refer(&mut $arg)
                    .add_argument($aname, argparse::Store, $ahelp)
                    .required();
            )*
            let (name, mut args) = $crate::cli::number_args();
            args.insert(0, name);
            match ap.parse(args, &mut ::std::io::stdout(), &mut ::std::io::stderr()) {
                Err(e) => ::std::process::exit(e),
                _ => ()
            }
        }
        $(
            let $opt = $opt;
        )*
        $(
            let $arg = $arg;
        )*
    }
}

fn prefix_number(args: &mut Vec<String>) {
    for i in 0..args.len() {
        let (notopt, num) = {
            let mut cs = args[i].chars();
            let opt = cs.next() == Some('-');
            let num = opt && match cs.next() {
                None => false,
                Some(c) => c >= '0' && c <= '9'
            };
            (!opt || num, num)
        };
        if num {
            args.insert(i, "--".to_string());
        }
        if notopt {
            break
        }
    }
}

/// Returns the basename of the running program
pub fn name(a: &mut env::Args) -> String {
    let path = a.next().unwrap();
    let filename = path::Path::new(&path).file_name().unwrap();
    let name = filename.to_str().unwrap().to_string();
    name
}

/// Returns the arguments and the basename of the running program
pub fn args() -> (String, Vec<String>) {
    let mut a = env::args();
    let name = name(&mut a);
    let args = a.collect();
    (name, args)
}

/// Returns the arguments and the basename of the rinning program
///
/// Inserts a `--` argument before the first argument that looks like a
/// negative number. Stops if it finds a non-option argument.
pub fn number_args() -> (String, Vec<String>) {
    let (name, mut args) = args();
    prefix_number(&mut args);
    (name, args)
}

/// Takes a `Result<T, E>` and writes an error message if it is `Err(E)`
///
/// Writes an error message and exits if it is `Err(E)`, returns `T` otherwise.
pub fn handle_error<T, E: fmt::Display>(name: &str, code: i32, r: Result<T, E>) -> T {
    match r {
        Ok(t) => t,
        Err(e) => {
            println_stderr!("{}: {}", name, e);
            process::exit(code);
        }
    }
}
