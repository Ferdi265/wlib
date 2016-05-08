use std::convert;
use std::str;
use std::mem;
use x11::xlib;

use super::display::Display;

pub struct WindowID(u64);

impl str::FromStr for WindowID {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<WindowID, Self::Err> {
        let mut hex = s.to_string();
        let is = if hex.len() < 3 {
            false
        } else {
            let pre: String = hex.drain(..2).collect();
            pre == "0x".to_string()
        };
        if is {
            u64::from_str_radix(&hex, 16).map_err(|_| "not a hexadecimal number").map(|u| u.into())
        } else {
            u64::from_str_radix(&hex, 10).map_err(|_| "not a decimal number").map(|u| u.into())
        }
    }
}

impl convert::From<u64> for WindowID {
    fn from(u: u64) -> WindowID {
        WindowID(u)
    }
}

impl convert::From<WindowID> for u64 {
    fn from(w: WindowID) -> u64 {
        w.0
    }
}

pub struct Window<'a> {
    w: u64,
    d: &'a Display<'a>,
    attrs: xlib::XWindowAttributes
}

impl<'a> Window<'a> {
    pub(super) fn new(d: &'a Display<'a>, id: WindowID) -> Result<Window<'a>, &'static str> {
        let mut w = Window {
            w: id.into(),
            d: d,
            attrs: unsafe {
                mem::zeroed()
            }
        };
        w.update_attrs().map(|_| w)
    }
    fn update_attrs(&mut self) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XGetWindowAttributes(mem::transmute(self.d.d), self.w, &mut self.attrs) == 1
        };
        if ok {
            Ok(())
        } else {
            Err("XGetWindowAttributes() failed")
        }
    }
    pub fn id(&self) -> u64 {
        self.w.into()
    }
    pub fn position(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XMoveWindow(mem::transmute(self.d.d), self.w, x, y) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XMoveWindow() failed")
        }
    }
    pub fn position_relative(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        let x = self.attrs.x + x;
        let y = self.attrs.y + y;
        self.position(x, y)
    }
    pub fn resize(&mut self, w: i32, h: i32) -> Result<(), &'static str> {
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
    pub fn resize_relative(&mut self, w: i32, h: i32) -> Result<(), &'static str> {
        let w = self.attrs.width + w;
        let h = self.attrs.height + h;
        self.resize(w, h)
    }
}
