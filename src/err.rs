use std::io::Write;
use std::process;

// PRIVATE

// error handling sugar
pub(super) trait IntoOrErrorStr<T> where T: Sized {
    fn or_error_str(self, &'static str) -> OrErrorStr<T>;
}

// impl for *const T
impl<T> IntoOrErrorStr<*const T> for *const T {
    fn or_error_str(self, e: &'static str) -> OrErrorStr<Self> {
        if self.is_null() {
            Err(e)
        } else {
            Ok(self)
        }
    }
}

// impl for *mut T
impl<T> IntoOrErrorStr<*mut T> for *mut T {
    fn or_error_str(self, e: &'static str) -> OrErrorStr<Self> {
        if self.is_null() {
            Err(e)
        } else {
            Ok(self)
        }
    }
}

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
