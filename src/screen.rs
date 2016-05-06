use std::mem;
use x11::xlib;

use super::err::Result;
use super::display::Display;
use super::window::Window;

pub struct Screen<'a> {
    d: &'a Display<'a>,
    s: &'a xlib::Screen
}

impl<'a> Screen<'a> {
    pub(super) fn new(d: &'a Display<'a>, s: &'a xlib::Screen) -> Screen<'a> {
        Screen {
            d: d,
            s: s
        }
    }
    pub fn root(&self) -> Result<Window<'a>> {
        Window::new(self.d, self.s.root)
    }
    pub fn width(&self) -> u16 {
        self.s.width as u16
    }
    pub fn height(&self) -> u16 {
        self.s.height as u16
    }
}

impl<'a> Drop for Screen<'a> {
    fn drop(&mut self) {
        let ok = unsafe {
            xlib::XFree(mem::transmute(self.s)) == 1
        };
        if !ok {
            panic!("XFree() failed");
        }
    }
}
