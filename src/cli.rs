use std::path;
use std::env;

fn prefix_number(args: &mut Vec<String>) {
    let mut i = 0;
    let mut len = args.len();
    while i < len {
        let (opt, num) = {
            let mut cs = args[i].chars();
            let opt = cs.next() == Some('-');
            let num = match cs.next() {
                None => false,
                Some(c) => c >= '0' && c <= '9'
            };
            (opt, num)
        };
        if !opt {
            break;
        } else if num {
            args.insert(i, "--".to_string());
            len += 1;
            i += 2;
        } else {
            i += 1;
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
