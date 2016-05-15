use std::mem;
use std::ptr;

use x11::xlib;

use super::Window;

pub struct Screen<'d> {
    d: &'d ptr::Unique<xlib::Display>,
    s: ptr::Unique<xlib::Screen>
}

impl<'d> Screen<'d> {
    pub(super) fn new(d: &'d ptr::Unique<xlib::Display>, s: ptr::Unique<xlib::Screen>) -> Self {
        Screen {
            d: d,
            s: s
        }
    }
    fn get(&self) -> &xlib::Screen {
        unsafe { self.s.get() }
    }
    /// Gets the root window of the screen
    ///
    /// Returns an error message if the root window does not exist.
    /// This should never happen.
    pub fn root(&self) -> Result<Window<'d>, &'static str> {
        Window::new(self.d, self.get().root.into())
    }
    /// Gets the width of the screen in pixels
    pub fn width(&self) -> i32 {
        self.get().width
    }
    /// Gets the height of the screen in pixels
    pub fn height(&self) -> i32 {
        self.get().height
    }
}

impl<'d> Drop for Screen<'d> {
    /// Frees the memory for the `Screen` when it is dropped
    ///
    /// Panics if the call to `XFree()` fails.
    /// This should never happen.
    fn drop(&mut self) {
        let ok = unsafe {
            xlib::XFree(mem::transmute(*self.s)) == 1
        };
        if !ok {
            panic!("XFree() failed");
        }
    }
}
