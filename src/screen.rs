use std::mem;
use std::ptr;

use x11::xlib;

use super::Display;
use super::Window;

pub struct Screen<'d> {
    d: &'d Display,
    s: ptr::Unique<xlib::Screen>
}

impl<'d> Screen<'d> {
    pub(super) fn new(d: &'d Display, s: ptr::Unique<xlib::Screen>) -> Self {
        Screen {
            d: d,
            s: s
        }
    }
    fn get(&self) -> &xlib::Screen {
        unsafe { self.s.get() }
    }
    pub(super) fn xlib_screen(&self) -> *mut xlib::Screen {
        *self.s
    }
    /// Returns the pointer coordinates relative to this screen's root window.
    ///
    /// Returns an error if the call to `XQueryPointer()` fails.
    pub fn pointer(&self) -> Result<(i32, i32), &'static str> {
        let win = try!(self.root());
        let ptr = try!(win.pointer_direct());
        Ok(ptr.pos)
    }
    /// Moves the pointer coordinates relative to this screen's root window.
    ///
    /// Returns an error if the call to `XQueryPointer()` fails.
    pub fn warp_pointer(&self, x: i32, y: i32) -> Result<(), &'static str> {
        let win = try!(self.root());
        win.warp_pointer(x, y)
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
    ///
    /// This function apparrently has no effect on screens as multiple screen
    /// pointers created by `XScreenOfDisplay()` are exactly equal and dropping
    /// one, which calls `XFree()` has no effect on the others. Strange.
    fn drop(&mut self) {
        let ok = unsafe {
            xlib::XFree(mem::transmute(self.xlib_screen())) == 1
        };
        if !ok {
            panic!("XFree() failed");
        }
    }
}
