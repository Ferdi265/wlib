use x11::xlib;

pub struct Screen {
    pub(super) s: *mut xlib::Screen
}

impl Screen {
    pub fn root(&self) -> xlib::Window {
        unsafe { (*self.s).root }
    }
    pub fn width(&self) -> i32 {
        unsafe { (*self.s).width as i32 }
    }
    pub fn height(&self) -> i32 {
        unsafe { (*self.s).height as i32 }
    }
}
