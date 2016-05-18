use std::ptr;

use x11::xlib;

use super::Display;
use super::Window;
use super::shapes;

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
    pub fn pointer(&self) -> Result<shapes::Point, &'static str> {
        let win = try!(self.root());
        let ptr = try!(win.pointer_direct());
        Ok(ptr.pos)
    }
    /// Moves the pointer coordinates relative to this screen's root window.
    ///
    /// Returns an error if the call to `XQueryPointer()` fails.
    pub fn warp_pointer(&self, p: shapes::Point) -> Result<(), &'static str> {
        let win = try!(self.root());
        win.warp_pointer(p)
    }
    /// Gets the root window of the screen
    ///
    /// Returns an error message if the root window does not exist.
    /// This should never happen.
    pub fn root(&self) -> Result<Window<'d>, &'static str> {
        Window::new(self.d, self.get().root.into())
    }
    /// Gets the width of the screen in pixels
    pub fn width(&self) -> u32 {
        self.get().width as u32
    }
    /// Gets the height of the screen in pixels
    pub fn height(&self) -> u32 {
        self.get().height as u32
    }
    pub fn rectangle(&self) -> shapes::PositionedRectangle {
        shapes::PositionedRectangle::new(0, 0, self.width(), self.height())
    }
}
