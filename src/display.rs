use std::os::raw;
use std::mem;
use std::ptr;
use std::ffi;
use x11::xlib;

use super::err::OrErrorStr;
use super::screen::Screen;

pub struct Display<'a> {
    pub(super) d: &'a xlib::Display
}

impl<'a> Display<'a> {
    fn open_direct(dispname: *const raw::c_char) -> OrErrorStr<Display<'a>> {
        let d = unsafe { xlib::XOpenDisplay(dispname).as_ref() };
        d.map(|d| Display { d: d }).ok_or("XOpenDisplay() failed: pointer is NULL")
    }
    pub fn open_named(dispname: &str) -> OrErrorStr<Display<'a>> {
        let cs = try!(
            ffi::CString::new(dispname)
                .map_err(|_| "CString::new() failed: found NULL byte")
        );
        return Self::open_direct(cs.as_ptr());
    }
    pub fn open() -> OrErrorStr<Display<'a>> {
        return Self::open_direct(ptr::null());
    }
    pub fn screen_num(&'a self, screennum: i32) -> OrErrorStr<Screen<'a>> {
        if screennum < 0 {
            return Err("screennum cannot be less than 0");
        }
        let count = unsafe { xlib::XScreenCount(mem::transmute(self.d)) };
        if screennum >= count {
            return Err("screennum greater than XScreenCount()");
        }
        let s = unsafe { xlib::XScreenOfDisplay(mem::transmute(self.d), screennum).as_ref() };
        s.map(|s| Screen { s: s, d: self }).ok_or("XScreenOfDisplay() failed: pointer is NULL")
    }
    pub fn screen(&'a self) -> OrErrorStr<Screen<'a>> {
        let s = unsafe { xlib::XDefaultScreenOfDisplay(mem::transmute(self.d)).as_ref() };
        s.map(|s| Screen { s: s, d: self }).ok_or("XDefaultScreenOfDisplay() failed: pointer is NULL")
    }
}

impl<'a> Drop for Display<'a> {
    // NOTE: XCloseDisplay() is hardcoded to return 0, so ignore it
    //
    // TODO: this crashes the program if there are errors. Maybe handle them?
    fn drop(&mut self) {
        unsafe { xlib::XCloseDisplay(mem::transmute(self.d)) };
    }
}