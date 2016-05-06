use std::io::Write;
use std::process;
use std::path;
use std::env;
use super::err::Result;

fn prefix_number(args: &mut Vec<String>) {
    for i in 0..args.len() {
        let (notopt, num) = {
            let mut cs = args[i].chars();
            let opt = cs.next() != Some('-');
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

pub fn args() -> (String, Vec<String>) {
    let mut a = env::args();
    let path = a.next().unwrap();
    let filename = path::Path::new(&path).file_name().unwrap();
    let name = filename.to_str().unwrap().to_string();
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

pub fn handle_error<T>(name: &str, code: i32, r: Result<T>) -> T {
    match r {
        Ok(t) => t,
        Err(e) => {
            println_stderr!("{}: {}", name, e);
            process::exit(code);
        }
    }
}
