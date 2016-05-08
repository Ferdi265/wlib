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
    pub fn open_named(dispname: &str) -> Result<Display<'a>, &'static str> {
        let cs = try!(
            ffi::CString::new(dispname)
                .map_err(|_| "CString::new() failed: found NULL byte")
        );
        return Self::open_direct(cs.as_ptr());
    }
    pub fn open() -> Result<Display<'a>, &'static str> {
        return Self::open_direct(ptr::null());
    }
    pub fn screen_num(&'a self, screennum: i32) -> Result<Screen<'a>, &'static str> {
        if screennum < 0 {
            return Err("screennum less than 0");
        }
        let count = unsafe {
            xlib::XScreenCount(mem::transmute(self.d))
        };
        if screennum >= count {
            return Err("screennum greater than XScreenCount()");
        }
        let s = unsafe {
            xlib::XScreenOfDisplay(mem::transmute(self.d), screennum).as_ref()
        };
        s.map(|s| Screen::new(self.d, s)).ok_or("XScreenOfDisplay() failed: pointer is NULL")
    }
    pub fn screen(&'a self) -> Result<Screen<'a>, &'static str> {
        let s = unsafe {
            xlib::XDefaultScreenOfDisplay(mem::transmute(self.d)).as_ref()
        };
        s.map(|s| Screen::new(self.d, s)).ok_or("XDefaultScreenOfDisplay() failed: pointer is NULL")
    }
    pub fn window(&'a self, id: WindowID) -> Result<Window<'a>, &'static str> {
        Window::new(self.d, id)
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
