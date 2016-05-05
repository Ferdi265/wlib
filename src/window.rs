use std::mem;
use x11::xlib;

use super::display::Display;

pub struct Window<'a> {
    pub(super) w: xlib::Window,
    pub(super) d: &'a Display<'a>
}

impl<'a> Window<'a> {
    pub fn id(&self) -> i32 {
        self.w as i32
    }
    pub fn translate(&self, x: i32, y: i32) {
        let r = unsafe { xlib::XMoveWindow(mem::transmute(self.d), self.w, x, y) };
        println!("XMoveWindow() = {}", r);
    }
}
