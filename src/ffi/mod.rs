#![allow(non_snake_case)]

use std::mem;
use std::ptr;
use std::ffi;
use x11::xlib;

pub mod types;

pub use super::ffi::types::*;

pub fn XOpenDisplay(name: Option<&ffi::CStr>) -> Option<XDisplay> {
    let name_ptr = match name {
        Some(n) => n.as_ptr(),
        None => ptr::null()
    };
    let ptr = unsafe { xlib::XOpenDisplay(name_ptr) };
    if ptr.is_null() {
        None
    } else {
        Some(XDisplay::new(unsafe { ptr::Shared::new(ptr) }))
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
        Some(XScreen::new(unsafe { ptr::Shared::new(ptr) }))
    }
}

pub fn XGetWindowAttributes(disp: &XDisplay, win: XWindow) -> Option<XWindowAttributes> {
    let mut attrs: xlib::XWindowAttributes = unsafe { mem::zeroed() };
    let ok = unsafe {
        xlib::XGetWindowAttributes(disp.ptr(), win, &mut attrs) > 0
    };
    if ok {
        Some(XWindowAttributes {
            x: attrs.x,
            y: attrs.y,
            w: attrs.width as u32,
            h: attrs.height as u32,
            b: attrs.border_width as u32,
            map_state: XMapState::from_raw(attrs.map_state),
            override_redirect: attrs.override_redirect > 0,
            root: attrs.root as XWindow,
            screen: XScreen::new(unsafe { ptr::Shared::new(attrs.screen) })
        })
    } else {
        None
    }
}
