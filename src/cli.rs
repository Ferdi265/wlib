use std::process;
use std::path;
use std::env;
use std::fmt;

#[macro_export]
macro_rules! println_stderr {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    }}
}

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

pub fn name(a: &mut env::Args) -> String {
    let path = a.next().unwrap();
    let filename = path::Path::new(&path).file_name().unwrap();
    let name = filename.to_str().unwrap().to_string();
    name
}

pub fn args() -> (String, Vec<String>) {
    let mut a = env::args();
    let name = name(&mut a);
    let args = a.collect();
    (name, args)
}

pub fn number_args() -> (String, Vec<String>) {
    let (name, mut args) = args();
    prefix_number(&mut args);
    (name, args)
}

pub fn parse_hex(h: &str) -> Option<u64> {
    let mut hex = h.to_string();
    let is = if hex.len() < 3 {
        false
    } else {
        let pre: String = hex.drain(..2).collect();
        pre == "0x".to_string()
    };
    if is {
        u64::from_str_radix(&hex, 16).ok()
    } else {
        None
    }
}

pub fn handle_error<T, E: fmt::Display>(name: &str, code: i32, r: Result<T, E>) -> T {
    match r {
        Ok(t) => t,
        Err(e) => {
            println_stderr!("{}: {}", name, e);
            process::exit(code);
        }
    }
}
