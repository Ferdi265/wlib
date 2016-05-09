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
    /// Returns an error message if the call to `XMoveWindow()` or the call to
    /// `XWindowAttributes()` after moving failed.
    pub fn reposition_absolute(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XMoveWindow(**self.d, self.w.into(), x, y) == 1
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
    pub fn reposition_relative(&mut self, x: i32, y: i32) -> Result<(), &'static str> {
        let x = self.x() + x;
        let y = self.y() + y;
        self.reposition_absolute(x, y)
    }
    /// Resizes the window
    ///
    /// Resizes the window to width `w` and height `h`.
    ///
    /// Returns an error message if the call to `XResizeWindow()` or the call to
    /// `XWindowAttributes()` after resizing failed.
    pub fn resize_absolute(&mut self, w: i32, h: i32) -> Result<(), &'static str> {
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
            xlib::XResizeWindow(**self.d, self.w.into(), w as u32, h as u32) == 1
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
        let w = self.width() + w;
        let h = self.height() + h;
        self.resize_absolute(w, h)
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
    pub fn configure<'w>(&'w mut self) -> Changes<'d, 'w> {
        Changes::new(self)
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
    pub fn border(&self) -> i32 {
        self.attrs.border_width
    }
    pub fn visible(&self) -> bool {
        self.attrs.map_state == xlib::IsViewable
    }
    pub fn mapped(&self) -> bool {
        self.attrs.map_state != xlib::IsUnmapped
    }
    pub fn ignored(&self) -> bool {
        self.attrs.override_redirect == 1
    }
}

pub enum StackMode {
    Above = 0,
    Below = 1,
    Opposite = 4
}

pub struct Changes<'d: 'w, 'w> {
    w: &'w mut Window<'d>,
    changes: xlib::XWindowChanges,
    mask: u16
}

impl<'d, 'w> Changes<'d, 'w> {
    fn new(w: &'w mut Window<'d>) -> Self {
        Changes {
            w: w,
            changes: unsafe { mem::zeroed() },
            mask: 0
        }
    }
    pub fn x(&mut self, x: i32) {
        self.changes.x = x;
        self.mask |= xlib::CWX;
    }
    pub fn y(&mut self, y: i32) {
        self.changes.y = y;
        self.mask |= xlib::CWY;
    }
    pub fn width(&mut self, width: i32) {
        self.changes.width = width;
        self.mask |= xlib::CWWidth;
    }
    pub fn height(&mut self, height: i32) {
        self.changes.height = height;
        self.mask |= xlib::CWHeight;
    }
    pub fn border(&mut self, border: i32) {
        self.changes.border_width = border;
        self.mask |= xlib::CWBorderWidth;
    }
    pub fn stack(&mut self, stack: StackMode) {
        self.changes.stack_mode = stack as i32;
        self.mask |= xlib::CWStackMode;
    }
    pub fn apply(self) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XConfigureWindow(**self.w.d, self.w.id().into(), self.mask as u32, mem::transmute(&self.changes)) == 1
        };
        if ok {
            self.w.update_attrs()
        } else {
            Err("XConfigureWindow() failed")
        }
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
