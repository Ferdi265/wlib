use std::convert;
use std::slice;
use std::fmt;
use std::str;
use std::mem;
use std::ptr;

use x11::xlib;

use super::Color;

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
    /// Returns an error message if the call to `XConfigureWindow()` or the
    /// call to `XGetWindowAttributes()` after moving failed.
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
    /// Returns an error message if the call to `XConfigureWindow()` or the
    /// call to `XGetWindowAttributes()` after moving failed.
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
    /// Returns an error message if the call to `XConfigureWindow()` or the
    /// call to `XGetWindowAttributes()` after resizing failed.
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
    /// Returns an error message if the call to `XConfigureWindow()` or the
    /// call to `XGetWindowAttributes()` after resizing failed.
    pub fn resize_relative(&mut self, w: i32, h: i32) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.width(self.width() + w);
        c.height(self.height() + h);
        self.change(&c)
    }
    /// Resizes the window border
    ///
    /// Resizes the window border to `b` pixels
    ///
    /// Returns an error message if the call to `XConfigureWindow()` or the
    /// call to `XGetWindowAttributes()` after resizing failed.
    pub fn border_resize(&mut self, b: i32) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.border_width(b);
        self.change(&c)
    }
    /// Changes the window border color
    ///
    /// Changes the window border to the color `color`
    ///
    /// Returns an error message if the call to `XChangeWindowAttributes()` or
    /// the call to `XGetWindowAttributes()` after recoloring failed.
    pub fn border_recolor(&mut self, color: Color) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.border_color(color);
        self.change(&c)
    }
    /// Changes ignore state for this window
    ///
    /// Sets or unsets override_redirect for this window
    ///
    /// Returns an error message if the call to `XChangeWindowAttributes()` or
    /// the call to XGetWindowAttributes()` afterwards failed.
    pub fn ignore(&mut self, ignore: bool) -> Result<(), &'static str> {
        let mut c = Changes::new();
        c.ignore(ignore);
        self.change(&c)
    }
    /// Maps the window
    ///
    /// Returns an error message if the call to `XMapWindow()` failed.
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
    /// Unmaps the window
    ///
    /// Returns an error message if the call to `XUnmapWindow()` failed.
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
    /// Changes window properties in batch
    ///
    /// Takes a `Changes` and calls `xlib` functions to apply these changes.
    ///
    /// Returns an error message if the call to either of these fails:
    ///
    /// - `XChangeWindowAttributes()` if `border_color` or `ignore` state are
    ///   changed
    /// - `XConfigureWindow()` if `x`, `y`, `width`, `height` or `border_width`
    ///   are changed
    /// - `XGetWindowAttributes()`
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
    /// Focuses the window
    ///
    /// Passes `RevertToPointerRoot` and `CurrentTime`.
    ///
    /// Returns an error message if the call to `XSetInputFocus()` failed.
    pub fn focus(&self) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XSetInputFocus(**self.d, self.id().into(), xlib::RevertToPointerRoot, xlib::CurrentTime) > 0
        };
        if ok {
            Ok(())
        } else {
            Err("XSetInputFocus() failed")
        }
    }
    /// Returns the children of the window
    ///
    /// Returns an error message if the call to `XQueryTree()` failed or if it
    /// returned NULL with an `n` greater than zero. Also returns errors if any
    /// of the children give errors at the `XGetWindowAttributes()` call.
    pub fn children(&self) -> Result<Vec<Window<'d>>, &'static str> {
        Ok(()).and_then(|_| {
            let mut _i = (0, 0);
            let mut n = 0;
            let mut ws = ptr::null_mut();
            let ok = unsafe {
                xlib::XQueryTree(**self.d, self.id().into(), &mut _i.0, &mut _i.1, &mut ws, &mut n) > 0
            };
            if ok {
                Ok((ws, n))
            } else {
                Err("XQueryTree() failed")
            }
        }).and_then(|res| {
            let (ws, n) = res;
            let mut children = vec![];
            if n == 0 {
                Ok(children)
            } else if ws.is_null() {
                Err("XQueryTree() returned less windows than it promised")
            } else {
                let windows = unsafe {
                    slice::from_raw_parts(ws, n as usize)
                };
                for w in windows {
                    let w = try!(Window::new(self.d, (*w).into()));
                    children.push(w);
                }
                Ok(children)
            }
        })
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

#[derive(Copy, Clone, Debug)]
pub struct ID(pub u64);

impl str::FromStr for ID {
    type Err = &'static str;
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

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}
