use std::os::raw;
use std::ptr;
use std::ffi;

use x11::xlib;

use super::Screen;
use super::Window;
use super::window;

unsafe extern "C" fn x_noop_error_handler(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> i32 {
    0
}

pub struct Display {
    pub(super) d: ptr::Unique<xlib::Display>
}

impl Display {
    fn new(d: ptr::Unique<xlib::Display>) -> Self {
        Display { d: d }
    }
    fn open_direct(dispname: *const raw::c_char) -> Result<Self, &'static str> {
        let d = unsafe {
            // NOTE: register noop error handler to avoid crashes
            xlib::XSetErrorHandler(Some(x_noop_error_handler));
            xlib::XOpenDisplay(dispname)
        };
        if d.is_null() {
            Err("XOpenDisplay() failed")
        } else {
            Ok(Display::new(unsafe { ptr::Unique::new(d) }))
        }
    }
    /// Opens a connection to the Xorg display server
    ///
    /// Opens the display `dispname`.
    ///
    /// Returns an error message if either `dispname` is not a valid
    /// `std::ffi::CString` or the call to `XOpenDisplay()` returned a NULL
    /// pointer.
    pub fn open_named(dispname: &'static str) -> Result<Self, &str> {
        let cs = try!(
            ffi::CString::new(dispname)
                .map_err(|_| "CString::new() failed")
        );
        return Self::open_direct(cs.as_ptr());
    }
    /// Opens a connection to the Xorg display server
    ///
    /// Opens the display given in the `$DISPLAY` environment variable.
    ///
    /// Returns an error message if the call to `XOpenDisplay()` returned a
    /// NULL pointer
    pub fn open() -> Result<Self, &'static str> {
        return Self::open_direct(ptr::null());
    }
    /// Get the number of screens the display has
    pub fn screens(&self) -> i32 {
        unsafe { xlib::XScreenCount(*self.d) }
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
    pub fn screen_num<'d>(&'d self, screennum: i32) -> Result<Screen<'d>, &'static str> {
        if screennum < 0 {
            return Err("screennum less than 0");
        }
        if screennum >= self.screens() {
            return Err("screennum greater or equal to screen count");
        }
        let s = unsafe {
            xlib::XScreenOfDisplay(*self.d, screennum)
        };
        if s.is_null() {
            Err("XScreenOfDisplay() failed")
        } else {
            Ok(Screen::new(&self.d, unsafe { ptr::Unique::new(s) }))
        }
    }
    /// Get the default screen associated with the display
    ///
    /// Returns an error message if the call to `XDefaultScreenOfDisplay`
    /// returned a NULL pointer.
    pub fn screen<'d>(&'d self) -> Result<Screen<'d>, &'static str> {
        let s = unsafe {
            xlib::XDefaultScreenOfDisplay(*self.d)
        };
        if s.is_null() {
            Err("XDefaultScreenOfDisplay() failed")
        } else {
            Ok(Screen::new(&self.d, unsafe { ptr::Unique::new(s) }))
        }
    }
    /// Gets the window with the specified window id
    ///
    /// Returns an error if the window does not exist.
    pub fn window<'d>(&'d self, id: window::ID) -> Result<Window<'d>, &'static str> {
        Window::new(&self.d, id)
    }
    /// Gets the currently focused window
    pub fn focus<'d>(&'d self) -> Result<Option<Window<'d>>, &'static str> {
        let mut id = 0;
        let mut revert = 0;
        let ok = unsafe {
            xlib::XGetInputFocus(*self.d, &mut id, &mut revert) > 0
        };
        if ok {
            const NONE: u64 = 0; /* xlib::None, which is commented out for no reason */
            const POINTER_ROOT: u64 = xlib::PointerRoot as u64;
            match id {
                NONE  => Ok(None),
                POINTER_ROOT => Ok(None),
                i => Window::new(&self.d, i.into()).map(|w| Some(w))
            }
        } else {
            Err("XGetInputFocus() failed")
        }
    }
}

impl Drop for Display {
    /// Closes the connection when the `Display` is dropped
    fn drop(&mut self) {
        unsafe {
            // NOTE: XCloseDisplay() is hardcoded to return 0, so ignore it
            xlib::XCloseDisplay(*self.d);
        }
    }
}
