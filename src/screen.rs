use std::mem;
use x11::xlib;

use super::Window;

pub struct Screen<'a> {
    d: &'a xlib::Display,
    s: &'a xlib::Screen
}

impl<'a> Screen<'a> {
    pub(super) fn new(d: &'a xlib::Display, s: &'a xlib::Screen) -> Screen<'a> {
        Screen {
            d: d,
            s: s
        }
    }
    /// Gets the root window of the screen
    ///
    /// Returns an error message if the root window does not exist.
    /// This should never happen.
    pub fn root(&self) -> Result<Window<'a>, &'static str> {
        Window::new(self.d, self.s.root.into())
    }
    /// Gets the width of the screen in pixels
    pub fn width(&self) -> i32 {
        self.s.width
    }
    /// Gets the height of the screen in pixels
    pub fn height(&self) -> i32 {
        self.s.height
    }
}

impl<'a> Drop for Screen<'a> {
    /// Frees the memory for the `Screen` when it is dropped
    ///
    /// Panics if the call to `XFree()` fails.
    /// This should never happen.
    fn drop(&mut self) {
        let ok = unsafe {
            xlib::XFree(mem::transmute(self.s)) == 1
        };
        if !ok {
            panic!("XFree() failed");
        }
    }
}
