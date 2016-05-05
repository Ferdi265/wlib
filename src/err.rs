use std::io::Write;
use std::process;

// PRIVATE

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

// PUBLIC

pub type OrErrorStr<T> = Result<T, &'static str>;

// error handling util for downstream
pub fn handle_error(e: OrErrorStr<()>) {
    if let Err(e) = e {
        println_stderr!("Error: {}", e);
        process::exit(1);
    }
}
