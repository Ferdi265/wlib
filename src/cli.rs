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
/// # fn main() {
/// #[derive(Copy, Clone, Debug)]
/// enum Mode {
///     Relative,
///     Absolute
/// }
///
/// parse_args!{
///     description: "a cli utility",
/// #   arguments: ("test".to_string(), vec![
/// #       "--relative".to_string(),
/// #       "-c".to_string(),
/// #       "0xdec0de".to_string(),
/// #       "213".to_string(),
/// #       "-15".to_string(),
/// #       "0xdeadbeef".to_string()
/// #   ]),
///     flag mode: Mode = Mode::Relative,
///         (&["-r", "--relative"], Mode::Relative, "do sth relatively"),
///         (&["-r", "--absolute"], Mode::Absolute, "do sth absolutely"),
///     opt color: wtools::Color,
///         (&["-c", "--color"], "color (hexadecimal)"),
///     arg x: i32,
///         ("x", "x coordinate"),
///     arg y: i32,
///         ("y", "y coordinate"),
///     arg wid: wtools::window::ID,
///         ("wid", "window id")
/// }
///
/// println!("mode: {:?}, color: {:?}, x: {}, y: {}, wid: {}", mode, color, x, y, wid);
/// # }
/// ```
#[macro_export]
macro_rules! parse_args {
    {
        description : $desc:expr
        $( , arguments : $arguments:expr )*
        $( , flag $flag:ident : $ftype:ty = $fdefault:expr ,
           $(( $fnames:expr , $fvalue:expr , $fhelp:expr )),*
        )*
        $( , list $list:ident : $ltype:ty ,
           $(( $lnames:expr , $lvalue:expr , $lhelp:expr )),*
        )*
        $( , opt $opt:ident : $otype:ty ,
           $(( $onames:expr , $ohelp:expr )),*
        )*
        $( , arg $arg:ident : $atype:ty ,
            ( $aname: expr , $ahelp:expr )
        )*
        $( , optarg $oarg:ident : $oatype:ty ,
            ( $oaname: expr , $oahelp:expr )
        )*
    } => {
        $( let mut $flag: $ftype = $fdefault; )*
        $( let mut $list: Vec<$ltype> = vec![]; )*
        $( let mut $opt: Option<$otype> = None; )*
        $( let mut $arg: Option<$atype> = None; )*
        $( let mut $oarg: Option<$oatype> = None; )*
        {
            extern crate argparse;
            let mut ap = argparse::ArgumentParser::new();
            ap.set_description($desc);
            ap.stop_on_first_argument(true);
            $( ap.refer(&mut $flag)
               $(.add_option($fnames, argparse::StoreConst($fvalue), $fhelp))*;
            )*
            $( ap.refer(&mut $list)
               $(.add_option($lnames, argparse::PushConst($lvalue), $lhelp))*;
            )*
            $( ap.refer(&mut $opt)
               $(.add_option($onames, argparse::StoreOption, $ohelp))*;
            )*
            $( ap.refer(&mut $arg)
                .add_argument($aname, argparse::StoreOption, $ahelp)
                .required();
            )*
            $( ap.refer(&mut $oarg)
                .add_argument($oaname, argparse::StoreOption, $oahelp);
            )*
            let mut arguments: Option<(String, Vec<String>)> = None;
            $(
                if arguments == None {
                    arguments = Some($arguments);
                }
            )*
            if arguments == None {
                arguments = Some($crate::cli::number_args());
            }
            let (name, mut args) = arguments.unwrap();
            args.insert(0, name);
            match ap.parse(args, &mut ::std::io::stdout(), &mut ::std::io::stderr()) {
                Err(e) => ::std::process::exit(e),
                _ => ()
            }
        }
        $( let $flag = $flag; )*
        $( let $list = $list; )*
        $( let $opt = $opt; )*
        $( let $arg = $arg.unwrap(); )*
        $( let $oarg = $oarg; )*
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

/// Returns the arguments and the basename of the running program
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
