use std::mem;
use x11::xlib;

use super::err::OrError;
use super::display::Display;
use super::window::Window;

pub struct Screen<'a> {
    pub(super) s: &'a xlib::Screen,
    pub(super) d: &'a Display<'a>
}

impl<'a> Screen<'a> {
    pub fn root(&self) -> OrError<Window<'a>> {
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
            panic!("XFree() failed: return was 0")   
        }
    }
}
