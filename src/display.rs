use std::os::raw;
use std::mem;
use std::ptr;
use std::ffi;
use x11::xlib;

use super::err::OrError;
use super::err::x_error_handler;
use super::screen::Screen;
use super::window::Window;

pub struct Display<'a> {
    pub(super) d: &'a xlib::Display
}

impl<'a> Display<'a> {
    fn open_direct(dispname: *const raw::c_char) -> OrError<Display<'a>> {
        let d = unsafe {
            // NOTE: register handler to avoid crashes
            xlib::XSetErrorHandler(Some(x_error_handler));
            xlib::XOpenDisplay(dispname).as_ref()
        };
        d.map(|d| Display { d: d }).ok_or("XOpenDisplay() failed: pointer is NULL".to_string())
    }
    pub fn open_named(dispname: &str) -> OrError<Display<'a>> {
        let cs = try!(
            ffi::CString::new(dispname)
                .map_err(|_| "CString::new() failed: found NULL byte".to_string())
        );
        return Self::open_direct(cs.as_ptr());
    }
    pub fn open() -> OrError<Display<'a>> {
        return Self::open_direct(ptr::null());
    }
    pub fn screen_num(&'a self, screennum: i32) -> OrError<Screen<'a>> {
        if screennum < 0 {
            return Err("screennum less than 0".to_string());
        }
        let count = unsafe {
            xlib::XScreenCount(mem::transmute(self.d))
        };
        if screennum >= count {
            return Err("screennum greater than XScreenCount()".to_string());
        }
        let s = unsafe {
            xlib::XScreenOfDisplay(mem::transmute(self.d), screennum).as_ref()
        };
        s.map(|s| Screen { s: s, d: self }).ok_or("XScreenOfDisplay() failed: pointer is NULL".to_string())
    }
    pub fn screen(&'a self) -> OrError<Screen<'a>> {
        let s = unsafe {
            xlib::XDefaultScreenOfDisplay(mem::transmute(self.d)).as_ref()
        };
        s.map(|s| Screen { s: s, d: self }).ok_or("XDefaultScreenOfDisplay() failed: pointer is NULL".to_string())
    }
    pub fn window(&'a self, id: u64) -> OrError<Window<'a>> {
        Window::new(self, id)
    }
}

impl<'a> Drop for Display<'a> {
    fn drop(&mut self) {
        unsafe {
            // NOTE: XCloseDisplay() is hardcoded to return 0, so ignore it
            xlib::XCloseDisplay(mem::transmute(self.d));
        }
    }
}
