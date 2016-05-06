use std::io::Write;
use std::process;
use x11::xlib;

// PRIVATE

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

pub(super) unsafe extern "C" fn x_error_handler(_: *mut xlib::Display, err: *mut xlib::XErrorEvent) -> i32 {
    let e = *err;
    println_stderr!("Error: code: {}", e.error_code);
    0
}

// PUBLIC

pub type OrErrorStr<T> = Result<T, &'static str>;

// error handling util for downstream
pub fn handle_error(e: OrErrorStr<()>) {
    if let Err(e) = e {
        println_stderr!("Error: {}", e);
        process::exit(1);
    }
}
