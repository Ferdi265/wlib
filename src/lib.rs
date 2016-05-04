extern crate x11;

use std::ptr;
use std::os::raw;
use std::ffi;
use x11::xlib;

pub struct Display {
    d: *mut xlib::Display
}

impl Display {
    // calls XOpenDisplay()
    //
    // TODO: errors?
    fn open_direct(dispname: *const raw::c_char) -> Result<Display, &'static str> {
        let d = unsafe { xlib::XOpenDisplay(dispname) };
        if d.is_null() {
            return Err("XOpenDisplay() failed: pointer is NULL");
        } else {
            return Ok(Display { d: d })
        }
    }
    // open the X Display named by dispname
    pub fn open_named(dispname: &str) -> Result<Display, &'static str> {
        let cs = try!(
            ffi::CString::new(dispname)
                .map_err(|_| "CString::new() failed: found NULL byte")
        );
        return Self::open_direct(cs.as_ptr());
    }
    // open the X Display in $DISPLAY
    pub fn open() -> Result<Display, &'static str> {
        return Self::open_direct(ptr::null());
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
