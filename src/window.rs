use std::mem;
use x11::xlib;

use super::err::OrError;
use super::display::Display;

pub struct Window<'a> {
    pub(super) w: u64,
    pub(super) d: &'a Display<'a>,
    pub(super) attrs: xlib::XWindowAttributes
}

impl<'a> Window<'a> {
    pub(super) fn new(d: &'a Display<'a>, id: u64) -> OrError<Window<'a>> {
        let mut w = Window {
            w: id,
            d: d,
            attrs: unsafe {
                mem::zeroed()
            }
        };
        w.update_attrs().map(|_| w)
    }
    pub(super) fn update_attrs(&mut self) -> OrError<()> {
        let ok = unsafe {
            xlib::XGetWindowAttributes(mem::transmute(self.d.d), self.w, &mut self.attrs) == 1
        };
        if ok {
            Ok(())
        } else {
            Err("XGetWindowAttributes() failed: return was 0".to_string())
        }
    }
    pub fn id(&self) -> i32 {
        self.w as i32
    }
    pub fn position(&mut self, x: i32, y: i32) -> OrError<()> {
        let ok = unsafe {
            xlib::XMoveWindow(mem::transmute(self.d.d), self.w, x, y) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XMoveWindow() failed: return was 0".to_string())
        }
    }
    pub fn position_relative(&mut self, x: i32, y: i32) -> OrError<()> {
        let x = self.attrs.x + x;
        let y = self.attrs.y + y;
        self.position(x, y)
    }
    pub fn resize(&mut self, w: u16, h: u16) -> OrError<()> {
        let ok = unsafe {
            xlib::XResizeWindow(mem::transmute(self.d.d), self.w, w as u32, h as u32) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XResizeWindow() failed: return was 0".to_string())
        }
    }
    pub fn resize_relative(&mut self, w: i32, h: i32) -> OrError<()> {
        if self.attrs.width + w < 0 {
            Err("resulting width less than 0".to_string())
        } else if self.attrs.width + w > u16::max_value() as i32 {
            Err("resulting width greater than u16::max_alue()".to_string())
        } else if self.attrs.height + h < 0 {
            Err("resulting height less than 0".to_string())
        } else if self.attrs.height + w > u16::max_value() as i32 {
            Err("resulting height greater than u16::max_value()".to_string())
        } else {
            let w = self.attrs.width + w;
            let h = self.attrs.height + h;
            self.resize(w as u16, h as u16)
        }
    }
}
