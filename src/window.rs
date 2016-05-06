use std::mem;
use x11::xlib;

use super::err::OrErrorStr;
use super::display::Display;

pub struct Window<'a> {
    pub(super) w: u64,
    pub(super) d: &'a Display<'a>,
    pub(super) attrs: xlib::XWindowAttributes
}

impl<'a> Window<'a> {
    pub(super) fn new(d: &'a Display<'a>, id: u64) -> OrErrorStr<Window<'a>> {
        let mut w = Window {
            w: id,
            d: d,
            attrs: unsafe { mem::zeroed() }
        };
        let r = unsafe { xlib::XGetWindowAttributes(mem::transmute(w.d.d), w.w, &mut w.attrs) };
        // NOTE: 0 is error
        if r == 0 {
            Err("XGetWindowAttributes() failed: return was 0")
        } else {
            Ok(w)
        }
    }
    pub fn id(&self) -> i32 {
        self.w as i32
    }
    pub fn position(&self, x: i32, y: i32) -> OrErrorStr<()> {
        let r = unsafe { xlib::XMoveWindow(mem::transmute(self.d.d), self.w, x, y) };
        // NOTE: 0 is error
        if r == 0 {
            Err("XMoveWindow() failed: return was 0")
        } else {
            Ok(())
        }
    }
    pub fn resize(&self, w: u16, h: u16) -> OrErrorStr<()> {
        let r = unsafe { xlib::XResizeWindow(mem::transmute(self.d.d), self.w, w as u32, h as u32) };
        // NOTE: 0 is error
        if r == 0 {
            Err("XResizeWindow() failed: return was 0")
        } else {
            Ok(())
        }
    }
}
