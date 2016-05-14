use std::convert;
use std::str;
use std::mem;
use std::ptr;
use x11::xlib;

pub struct Window<'d> {
    w: ID,
    d: &'d ptr::Unique<xlib::Display>,
    attrs: xlib::XWindowAttributes
}

impl<'d> Window<'d> {
    pub(super) fn new(d: &'d ptr::Unique<xlib::Display>, id: ID) -> Result<Self, &'static str> {
        let mut w = Window {
            w: id,
            d: d,
            attrs: unsafe {
                mem::zeroed()
            }
        };
        w.update_attrs().map(|_| w)
    }
    fn update_attrs(&mut self) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XGetWindowAttributes(**self.d, self.w.into(), &mut self.attrs) == 1
        };
        if ok {
            Ok(())
        } else {
            Err("XGetWindowAttributes() failed")
        }
    }
    /// Moves the window
    ///
    /// Moves the window to the coordinates `x` and `y`.
    ///
    /// Returns an error message if the call to `XConfigureWindow()` or the call to
    /// `XWindowAttributes()` after moving failed.
    pub fn reposition_absolute(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.x(x);
        c.y(y);
        self.change(&c)
    }
    /// Moves the window relatively
    ///
    /// Moves the window by `x` pixels horizontally and `y` pixels vertically.
    ///
    /// Returns an error message if the call to `XConfigureWindow()` or the call to
    /// `XWindowAttributes()` after moving failed.
    pub fn reposition_relative(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.x(self.x() + x);
        c.y(self.y() + y);
        self.change(&c)
    }
    /// Resizes the window
    ///
    /// Resizes the window to width `w` and height `h`.
    ///
    /// Returns an error message if the call to `XConfigureWindow()` or the call to
    /// `XWindowAttributes()` after resizing failed.
    pub fn resize_absolute(&mut self, w: i32, h: i32) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.width(w);
        c.height(h);
        self.change(&c)
    }
    /// Resizes the window relatively
    ///
    /// Resizes the window by `w` pixels horizontally and `h` pixels
    /// vertically.
    ///
    /// Returns an error message if the call to `XConfigureWindow()` or the call to
    /// `XWindowAttributes()` after resizing failed.
    pub fn resize_relative(&mut self, w: i32, h: i32) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.width(self.width() + w);
        c.height(self.height() + h);
        self.change(&c)
    }
    pub fn border_resize(&mut self, b: i32) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.border_width(b);
        self.change(&c)
    }
    pub fn border_recolor(&mut self, color: Color) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.border_color(color);
        self.change(&c)
    }
    pub fn ignore(&mut self, ignore: bool) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.ignore(ignore);
        self.change(&c)
    }
    pub fn map(&mut self) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XMapWindow(**self.d, self.w.into()) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XMapWindow() failed")
        }
    }
    pub fn unmap(&mut self) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XUnmapWindow(**self.d, self.w.into()) == 1
        };
        if ok {
            self.update_attrs()
        } else {
            Err("XUnmapWindow() failed")
        }
    }
    pub fn change<'w>(&'w mut self, c: &Changes) -> Result<(), &'static str> {
        Ok(()).and_then(|_| {
            let mut attrs = c.attrs;
            let ok = unsafe {
                xlib::XChangeWindowAttributes(**self.d, self.id().into(), c.amask, &mut attrs) == 1
            };
            if ok {
                Ok(())
            } else {
                Err("XChangeWindowAttributes() failed")
            }
        }).and_then(|_| {
            let mut changes = c.changes;
            let ok = unsafe {
                xlib::XConfigureWindow(**self.d, self.id().into(), c.cmask as u32, &mut changes) == 1
            };
            if ok {
                Ok(())
            } else {
                Err("XConfigureWindow() failed")
            }
        }).and_then(|_| self.update_attrs())
    }
    pub fn id(&self) -> ID {
        self.w.into()
    }
    pub fn x(&self) -> i32 {
        self.attrs.x
    }
    pub fn y(&self) -> i32 {
        self.attrs.y
    }
    pub fn width(&self) -> i32 {
        self.attrs.width
    }
    pub fn height(&self) -> i32 {
        self.attrs.height
    }
    pub fn position(&self) -> (i32, i32) {
        (self.x(), self.y())
    }
    pub fn size(&self) -> (i32, i32) {
        (self.width(), self.height())
    }
    pub fn border_width(&self) -> i32 {
        self.attrs.border_width
    }
    pub fn ignored(&self) -> bool {
        self.attrs.override_redirect == 1
    }
    pub fn visible(&self) -> bool {
        self.attrs.map_state == xlib::IsViewable
    }
    pub fn mapped(&self) -> bool {
        self.attrs.map_state != xlib::IsUnmapped
    }
}

pub enum StackMode {
    Above = 0,
    Below = 1,
    Opposite = 4
}

pub struct Changes {
    changes: xlib::XWindowChanges,
    attrs: xlib::XSetWindowAttributes,
    cmask: u16,
    amask: u64
}

impl Changes {
    pub fn new() -> Self {
        Changes {
            changes: unsafe { mem::zeroed() },
            attrs: unsafe { mem::zeroed() },
            cmask: 0,
            amask: 0
        }
    }
    pub fn reset(&mut self) {
        self.changes = unsafe { mem::zeroed() };
        self.attrs = unsafe { mem::zeroed() };
        self.cmask = 0;
        self.amask = 0;
    }
    pub fn x(&mut self, x: i32) {
        self.changes.x = x;
        self.cmask |= xlib::CWX;
    }
    pub fn y(&mut self, y: i32) {
        self.changes.y = y;
        self.cmask |= xlib::CWY;
    }
    pub fn width(&mut self, width: i32) {
        self.changes.width = width;
        self.cmask |= xlib::CWWidth;
    }
    pub fn height(&mut self, height: i32) {
        self.changes.height = height;
        self.cmask |= xlib::CWHeight;
    }
    pub fn border_width(&mut self, border_width: i32) {
        self.changes.border_width = border_width;
        self.cmask |= xlib::CWBorderWidth;
    }
    pub fn stack(&mut self, stack: StackMode) {
        self.changes.stack_mode = stack as i32;
        self.cmask |= xlib::CWStackMode;
    }
    pub fn border_color(&mut self, border_color: Color) {
        let c: i32 = border_color.into();
        self.attrs.border_pixel = c as u64;
        self.amask |= xlib::CWBorderPixel;
    }
    pub fn ignore(&mut self, ignore: bool) {
        self.attrs.override_redirect = ignore as i32;
        self.amask |= xlib::CWOverrideRedirect;
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn from_i32(i: i32) -> Self {
        Color {
            r: (i & (255 << 8 * 2)) as u8,
            g: (i & (255 << 8 * 1)) as u8,
            b: (i & (255 << 8 * 0)) as u8
        }
    }
}

impl str::FromStr for Color {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hex = s.to_string();
        let is = if hex.len() != 8 {
            false
        } else {
            let pre: String = hex.drain(..2).collect();
            pre == "0x".to_string()
        };
        if is {
            i32::from_str_radix(&hex, 16).map_err(|_| "not a hexadecimal color").map(|i| Color::from_i32(i))
        } else {
            Err("not a hexadecimal color")
        }
    }
}

impl convert::Into<i32> for Color {
    fn into(self) -> i32 {
        0 |
            ((self.r as i32) << 8 * 2) |
            ((self.g as i32) << 8 * 1) |
            ((self.b as i32) << 8 * 0)
    }
}

#[derive(Copy, Clone)]
pub struct ID(pub u64);

impl str::FromStr for ID {
    type Err = &'static str;
    /// Creates a `ID` from a hexadecimal `&'static str`
    ///
    /// Parses a `&'static str` prefixed with `0x` as a hexadecimal number.
    ///
    /// Returns an error message if the `&'static str` was malformed.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

impl convert::From<u64> for ID {
    fn from(u: u64) -> ID {
        ID(u)
    }
}

impl convert::Into<u64> for ID {
    fn into(self) -> u64 {
        self.0
    }
}
