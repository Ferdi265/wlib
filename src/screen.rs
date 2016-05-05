use std::mem;
use x11::xlib;

use super::display::Display;
use super::window::Window;

pub struct Screen<'a> {
    pub(super) s: &'a xlib::Screen,
    pub(super) d: &'a Display<'a>
}

impl<'a> Screen<'a> {
    pub fn root(&self) -> Window<'a> {
        Window { w: self.s.root, d: self.d }
    }
    pub fn width(&self) -> i32 {
        self.s.width as i32
    }
    pub fn height(&self) -> i32 {
        self.s.height as i32
    }
}

impl<'a> Drop for Screen<'a> {
    // TODO: find out what XFree() returns
    fn drop(&mut self) {
        let r = unsafe { xlib::XFree(mem::transmute(self.s)) };
        println!("XFree(screen) = {}", r);
    }
}
