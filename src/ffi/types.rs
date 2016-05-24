use std::ptr;
use x11::xlib;

#[derive(Eq)]
pub struct XDisplay {
    ptr: ptr::Shared<xlib::Display>
}

impl XDisplay {
    pub(super) fn new(ptr: ptr::Shared<xlib::Display>) -> XDisplay {
        XDisplay { ptr: ptr }
    }
    pub(super::super) fn ptr(&self) -> *mut xlib::Display {
        *self.ptr
    }
}

impl PartialEq for XDisplay {
    fn eq(&self, other: &XDisplay) -> bool {
        self.ptr() == other.ptr()
    }
}

impl Drop for XDisplay {
    fn drop(&mut self) {
         unsafe { xlib::XCloseDisplay(self.ptr()) };
    }
}

#[derive(Clone, Eq)]
pub struct XScreen {
    ptr: ptr::Shared<xlib::Screen>
}

impl XScreen {
    pub(super) fn new(ptr: ptr::Shared<xlib::Screen>) -> XScreen {
        XScreen { ptr: ptr }
    }
    pub(super::super) fn ptr(&self) -> *mut xlib::Screen {
        *self.ptr
    }
}

impl PartialEq for XScreen {
    fn eq(&self, other: &XScreen) -> bool {
        self.ptr() == other.ptr()
    }
}

pub type XWindow = u64;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum XMapState {
    Unmapped,
    Unviewable,
    Viewable
}

impl XMapState {
    pub fn from_raw(i: i32) -> XMapState {
        match i {
            xlib::IsUnmapped => XMapState::Unmapped,
            xlib::IsUnviewable => XMapState::Unviewable,
            xlib::IsViewable => XMapState::Viewable,
            _ => panic!("illegal map state")
        }
    }
    pub fn mapped(&self) -> bool {
        *self != XMapState::Unmapped
    }
    pub fn visible(&self) -> bool {
        *self == XMapState::Viewable
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct XWindowAttributes {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub b: u32,
    pub map_state: XMapState,
    pub override_redirect: bool,
    pub root: XWindow,
    pub screen: XScreen
}
