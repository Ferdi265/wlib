use std::mem;
use x11::xlib;

use super::err::Result;
use super::display::Display;

pub struct Window<'a> {
    w: u64,
    d: &'a Display<'a>,
    attrs: xlib::XWindowAttributes
}

impl<'a> Window<'a> {
    pub(super) fn new(d: &'a Display<'a>, id: u64) -> Result<Window<'a>> {
        let mut w = Window {
            w: id,
            d: d,
            attrs: unsafe {
                mem::zeroed()
            }
        };
        w.update_attrs().map(|_| w)
    }
    fn update_attrs(&mut self) -> Result<()> {
        let ok = unsafe {
            xlib::XGetWindowAttributes(mem::transmute(self.d.d), self.w, &mut self.attrs) == 1
        };
        if ok {
            Ok(())
        } else {
            Err("XGetWindowAttributes() failed")
        }
    }
    pub fn id(&self) -> i32 {
        self.w as i32
    }
    pub fn position(&mut self, x: i32, y: i32) -> Result<()> {
        let ok = unsafe {
            xlib::XMoveWindow(mem::transmute(self.d.d), self.w, x, y) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XMoveWindow() failed")
        }
    }
    pub fn position_relative(&mut self, x: i32, y: i32) -> Result<()> {
        let x = self.attrs.x + x;
        let y = self.attrs.y + y;
        self.position(x, y)
    }
    pub fn resize(&mut self, w: i32, h: i32) -> Result<()> {
        if w < 0 {
            return Err("width less than 0");
        } else if w > u16::max_value() as i32 {
            return Err("width greater than u16::max_alue()");
        } else if h < 0 {
            return Err("height less than 0");
        } else if w > u16::max_value() as i32 {
            return Err("height greater than u16::max_value()");
        }

        let ok = unsafe {
            xlib::XResizeWindow(mem::transmute(self.d.d), self.w, w as u32, h as u32) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XResizeWindow() failed")
        }
    }
    pub fn resize_relative(&mut self, w: i32, h: i32) -> Result<()> {
        let w = self.attrs.width + w;
        let h = self.attrs.height + h;
        self.resize(w, h)
    }
}
