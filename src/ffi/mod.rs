use std::ptr;
use std::ffi;
use x11::xlib;

pub mod types;

pub use super::ffi::types::*;

pub fn XOpenDisplay(name: Option<&ffi::CStr>) -> Option<XDisplay> {
    let name_ptr = match name {
        Some(n) => n,
        None => ptr::null()
    };
    let ptr = unsafe { xlib::XOpenDisplay(name_ptr) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { ptr::Unique::new(ptr) })
    }
}
pub fn XDefaultScreen(disp: &XDisplay) -> i32 {
    unsafe { xlib::XDefaultScreen(disp.ptr()) }
}
pub fn XScreenOfDisplay(disp: &XDisplay, num: i32) -> Option<XScreen> {
    let ptr = unsafe { xlib::XScreenOfDisplay(disp.ptr(), num) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { ptr::Unique::new(ptr))
    }
}

