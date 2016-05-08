use std::convert;
use std::str;
use std::mem;
use x11::xlib;

pub struct Window<'a> {
    w: u64,
    d: &'a xlib::Display,
    attrs: xlib::XWindowAttributes
}

impl<'a> Window<'a> {
    pub(super) fn new(d: &'a xlib::Display, id: WindowID) -> Result<Window<'a>, &'static str> {
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
            xlib::XGetWindowAttributes(mem::transmute(self.d), self.w, &mut self.attrs) == 1
        };
        if ok {
            Ok(())
        } else {
            Err("XGetWindowAttributes() failed")
        }
    }
    /// Returns the window id
    pub fn id(&self) -> WindowID {
        self.w.into()
    }
    /// Moves the window
    ///
    /// Moves the window to the coordinates `x` and `y`.
    ///
    /// Returns an error message if the call to `XMoveWindow()` or the call to
    /// `XWindowAttributes()` after moving failed.
    pub fn position(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XMoveWindow(mem::transmute(self.d), self.w, x, y) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XMoveWindow() failed")
        }
    }
    /// Moves the window relatively
    ///
    /// Moves the window by `x` pixels horizontally and `y` pixels vertically.
    ///
    /// Returns an error message if the call to `XMoveWindow()` or the call to
    /// `XWindowAttributes()` after moving failed.
    pub fn position_relative(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        let x = self.attrs.x + x;
        let y = self.attrs.y + y;
        self.position(x, y)
    }
    /// Resizes the window
    ///
    /// Resizes the window to width `w` and height `h`.
    ///
    /// Returns an error message if the call to `XResizeWindow()` or the call to
    /// `XWindowAttributes()` after resizing failed.
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
            xlib::XResizeWindow(mem::transmute(self.d), self.w, w as u32, h as u32) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XResizeWindow() failed")
        }
    }
    /// Resizes the window relatively
    ///
    /// Resizes the window by `w` pixels horizontally and `h` pixels
    /// vertically.
    ///
    /// Returns an error message if the call to `XResizeWindow()` or the call to
    /// `XWindowAttributes()` after resizing failed.
    pub fn resize_relative(&mut self, w: i32, h: i32) -> Result<(), &'static str> {
        let w = self.attrs.width + w;
        let h = self.attrs.height + h;
        self.resize(w, h)
    }
}

pub struct WindowID(pub u64);

impl str::FromStr for WindowID {
    type Err = &'static str;
    /// Creates a `WindowID` from a hexadecimal `&str`
    ///
    /// Parses a `&str` prefixed with `0x` as a hexadecimal number.
    ///
    /// Returns an error message if the `&str` was malformed.
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
            Err("not a hexadecimal number")
        }
    }
}

impl convert::From<u64> for WindowID {
    fn from(u: u64) -> WindowID {
        WindowID(u)
    }
}

impl convert::Into<u64> for WindowID {
    fn into(self) -> u64 {
        self.0
    }
}
