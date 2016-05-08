use std::os::raw;
use std::mem;
use std::ptr;
use std::ffi;
use x11::xlib;

use super::Screen;
use super::Window;
use super::WindowID;

unsafe extern "C" fn x_noop_error_handler(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> i32 {
    0
}

pub struct Display<'a> {
    pub(super) d: &'a xlib::Display
}

impl<'a> Display<'a> {
    fn open_direct(dispname: *const raw::c_char) -> Result<Display<'a>, &'static str> {
        let d = unsafe {
            // NOTE: register noop error handler to avoid crashes
            xlib::XSetErrorHandler(Some(x_noop_error_handler));
            xlib::XOpenDisplay(dispname).as_ref()
        };
        d.map(|d| Display { d: d }).ok_or("XOpenDisplay() failed: pointer is NULL")
    }
    /// Opens a connection to the Xorg display server
    ///
    /// Opens the display `dispname`.
    ///
    /// Returns an error message if either `dispname` is not a valid
    /// `std::ffi::CString` or the call to `XOpenDisplay()` returned a NULL
    /// pointer.
    pub fn open_named(dispname: &str) -> Result<Display<'a>, &'static str> {
        let cs = try!(
            ffi::CString::new(dispname)
                .map_err(|_| "CString::new() failed: found NULL byte")
        );
        return Self::open_direct(cs.as_ptr());
    }
    /// Opens a connection to the Xorg display server
    ///
    /// Opens the display given in the `$DISPLAY` environment variable.
    ///
    /// Returns an error message if the call to `XOpenDisplay()` returned a
    /// NULL pointer
    pub fn open() -> Result<Display<'a>, &'static str> {
        return Self::open_direct(ptr::null());
    }
    /// Get the number of screens the display has
    pub fn screens(&'a self) -> i32 {
        unsafe { xlib::XScreenCount(mem::transmute(self.d)) }
    }
    /// Get a screen from the display
    ///
    /// Gets the screen with the number `screennum` from the display
    ///
    /// Returns an error message if any of these statements are true:
    ///
    /// - `screennum` is less than 0
    /// - `screennum` is greater or equal to `display.screens()`
    /// - `XScreenOfDisplay()` returned a NULL pointer
    pub fn screen_num(&'a self, screennum: i32) -> Result<Screen<'a>, &'static str> {
        if screennum < 0 {
            return Err("screennum less than 0");
        }
        if screennum >= self.screens() {
            return Err("screennum greater or equal to screen count");
        }
        let s = unsafe {
            xlib::XScreenOfDisplay(mem::transmute(self.d), screennum).as_ref()
        };
        s.map(|s| Screen::new(self.d, s)).ok_or("XScreenOfDisplay() failed: pointer is NULL")
    }
    /// Get the default screen associated with the display
    ///
    /// Returns an error message if the call to `XDefaultScreenOfDisplay`
    /// returned a NULL pointer.
    pub fn screen(&'a self) -> Result<Screen<'a>, &'static str> {
        let s = unsafe {
            xlib::XDefaultScreenOfDisplay(mem::transmute(self.d)).as_ref()
        };
        s.map(|s| Screen::new(self.d, s)).ok_or("XDefaultScreenOfDisplay() failed: pointer is NULL")
    }
    /// Gets the window with the specified window id
    ///
    /// Returns an error if the window does not exist.
    pub fn window(&'a self, id: WindowID) -> Result<Window<'a>, &'static str> {
        Window::new(self.d, id)
    }
}

impl<'a> Drop for Display<'a> {
    /// Closes the connection when the `Display` is dropped
    fn drop(&mut self) {
        unsafe {
            // NOTE: XCloseDisplay() is hardcoded to return 0, so ignore it
            xlib::XCloseDisplay(mem::transmute(self.d));
        }
    }
}
