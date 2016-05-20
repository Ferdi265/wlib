use std::ptr;
use x11::xlib;

pub struct XDisplay {
    ptr: ptr::Unique<xlib::Display>
}

impl XDisplay {
    pub(super) fn new(ptr: ptr::Unique<xlib::Display>) -> XDisplay {
        XDisplay { ptr: ptr }
    }
    pub(super) fn ptr(&self) -> *mut xlib::Display {
        *self.ptr
    }
}

impl Drop for XDisplay {
    fn drop(&mut self) {
         unsafe { xlib::XCloseDisplay(self.ptr()) }
    }
}

pub struct XScreen {
    ptr: ptr::Unique<xlib::Screen>
}

impl XScreen {
    pub(super) fn new(ptr: ptr::Unique<xlib::Screen>) -> XScreen {
        XScreen { ptr: ptr }
    }
    pub(super) fn ptr(&self) -> *mut xlib::Screen {
        *self.ptr
    }
}

pub struct XWindow {
    id: u64
}
