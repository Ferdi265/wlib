use std::os::raw;
use std::ptr;
use std::ffi;
use x11::xlib;

use super::err::OrErrorStr;
use super::err::IntoOrErrorStr;
use super::screen::Screen;

// Display wrapper struct
pub struct Display {
    pub(super) d: *mut xlib::Display
}

impl Display {
    // calls XOpenDisplay()
    //
    // TODO: errors?
    fn open_direct(dispname: *const raw::c_char) -> OrErrorStr<Display> {
        let d = unsafe { xlib::XOpenDisplay(dispname) };
        d.or_error_str("XOpenDisplay() failed: pointer is NULL").map(|d| Display { d: d })
    }
    // open the X Display named by dispname
    pub fn open_named(dispname: &str) -> OrErrorStr<Display> {
        let cs = try!(
            ffi::CString::new(dispname)
                .map_err(|_| "CString::new() failed: found NULL byte")
        );
        return Self::open_direct(cs.as_ptr());
    }
    // open the X Display in $DISPLAY
    pub fn open() -> OrErrorStr<Display> {
        return Self::open_direct(ptr::null());
    }
    // get the screen with number num
    // calls XScreenOfDisplay()
    pub fn screen_num(&self, screennum: i32) -> OrErrorStr<Screen> {
        if screennum < 0 {
            return Err("screennum cannot be less than 0");
        }
        let count = unsafe { xlib::XScreenCount(self.d) };
        if screennum >= count {
            return Err("screennum greater than XScreenCount()");
        }
        let s = unsafe { xlib::XScreenOfDisplay(self.d, screennum) };
        s.or_error_str("XScreenOfDisplay() failed: pointer is NULL").map(|s| Screen { s: s })
    }
    // get the default screen
    // calls XDefaultScreenOfDisplay()
    pub fn screen(&self) -> OrErrorStr<Screen> {
        let s = unsafe { xlib::XDefaultScreenOfDisplay(self.d) };
        s.or_error_str("XDefaultScreenOfDisplay() failed: pointer is NULL").map(|s| Screen { s: s })
    }
}

impl Drop for Display {
    // calls XCloseDisplay()
    //
    // NOTE: XCloseDisplay() is hardcoded to return 0, so ignore it
    //
    // TODO: man page talks about errors?
    fn drop(&mut self) {
        unsafe { xlib::XCloseDisplay(self.d) };
    }
}
