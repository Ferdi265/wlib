use std::os::raw;
use std::ptr;
use std::ffi;

use x11::xlib;

use super::Screen;
use super::Window;
use super::window;
use super::shapes;

unsafe extern "C" fn x_noop_error_handler(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> i32 {
    0
}

pub struct Display {
    d: ptr::Unique<xlib::Display>
}

impl Display {
    fn new(d: ptr::Unique<xlib::Display>) -> Self {
        Display { d: d }
    }
    fn open_direct(dispname: *const raw::c_char) -> Result<Self, &'static str> {
        let d = unsafe {
            // NOTE: register noop error handler to avoid crashes
            xlib::XSetErrorHandler(Some(x_noop_error_handler));
            xlib::XOpenDisplay(dispname)
        };
        if d.is_null() {
            Err("XOpenDisplay() failed")
        } else {
            Ok(Display::new(unsafe { ptr::Unique::new(d) }))
        }
    }
    /// Opens a connection to the Xorg display server
    ///
    /// Opens the display `dispname`.
    ///
    /// Returns an error message if either `dispname` is not a valid
    /// `std::ffi::CString` or the call to `XOpenDisplay()` returned a NULL
    /// pointer.
    pub fn open_named(dispname: &str) -> Result<Self, &'static str> {
        let cs = try!(
            ffi::CString::new(dispname)
                .map_err(|_| "CString::new() failed")
        );
        return Self::open_direct(cs.as_ptr());
    }
    /// Opens a connection to the Xorg display server
    ///
    /// Opens the display given in the `$DISPLAY` environment variable.
    ///
    /// Returns an error message if the call to `XOpenDisplay()` returned a
    /// NULL pointer
    pub fn open() -> Result<Self, &'static str> {
        return Self::open_direct(ptr::null());
    }
    pub(super) fn xlib_display(&self) -> *mut xlib::Display {
        *self.d
    }
    pub(super) fn pointer_direct(&self, w: &Window) -> Result<Pointer, &'static str> {
        let mut root = 0;
        let mut _c = 0;
        let mut pos = shapes::Point::new(0, 0);
        let mut wpos = shapes::Point::new(0, 0);
        let mut _m = 0;
        let same_screen = unsafe {
            xlib::XQueryPointer(self.xlib_display(), w.id().into(), &mut root, &mut _c, &mut pos.x, &mut pos.y, &mut wpos.x, &mut wpos.y, &mut _m) > 0
        };
        if root == 0 /* xlib::None */ {
            Err("XQueryPointer() failed")
        } else {
            let wpos = if same_screen {
                Some(wpos)
            } else {
                None
            };
            Ok(Pointer {
                pos: pos,
                wpos: wpos
            })
        }
    }
    /// Pointer coordinates
    ///
    /// Returns an error if the call to `XQueryPointer()` fails.
    pub fn pointer(&self) -> Result<shapes::Point, &'static str> {
        let scrn = try!(self.screen());
        scrn.pointer()
    }
    /// Moves pointer absolutely
    ///
    /// Returns an error if the call to `XWarpPointer()` fails.
    pub fn warp_pointer_absolute(&self, p: shapes::Point) -> Result<(), &'static str> {
        let scrn = try!(self.screen());
        scrn.warp_pointer(p)
    }
    /// Moves pointer absolutely
    ///
    /// Returns an error if the call to `XWarpPointer()` fails.
    pub fn warp_pointer_relative(&self, p: shapes::Point) -> Result<(), &'static str> {
        let ok = unsafe {
            xlib::XWarpPointer(self.xlib_display(), 0 /* xlib::None */, 0 /* xlib::None */, 0, 0, 0, 0, p.x, p.y) > 0
        };
        if ok {
            Ok(())
        } else {
            Err("XWarpPointer() failed")
        }
    }
    /// Get the number of screens the display has
    pub fn screens(&self) -> u32 {
        unsafe { xlib::XScreenCount(self.xlib_display()) as u32 }
    }
    pub fn screen_default(&self) -> u32 {
        unsafe { xlib::XDefaultScreen(self.xlib_display()) as u32 }
    }
    /// Get a screen from the display
    ///
    /// Gets the screen with the number `screennum` from the display
    ///
    /// Returns an error message if the call to `XScreenOfDisplay()` returned a
    /// NULL pointer.
    pub fn screen_num<'d>(&'d self, screennum: u32) -> Result<Screen<'d>, &'static str> {
        let s = unsafe {
            xlib::XScreenOfDisplay(self.xlib_display(), screennum as i32)
        };
        if s.is_null() {
            Err("XScreenOfDisplay() failed")
        } else {
            Ok(Screen::new(&self, unsafe { ptr::Unique::new(s) }))
        }
    }
    /// Get the default screen associated with the display
    ///
    /// Returns an error message if the call to `XScreenOfDisplay()` returned a
    /// NULL pointer.
    pub fn screen<'d>(&'d self) -> Result<Screen<'d>, &'static str> {
        self.screen_num(self.screen_default())
    }
    /// Gets the window with the specified window id
    ///
    /// Returns an error if the window does not exist.
    pub fn window<'d>(&'d self, id: window::ID) -> Result<Window<'d>, &'static str> {
        Window::new(&self, id)
    }
    /// Gets the currently focused window
    pub fn focus<'d>(&'d self) -> Result<Option<Window<'d>>, &'static str> {
        let mut id = 0;
        let mut revert = 0;
        let ok = unsafe {
            xlib::XGetInputFocus(*self.d, &mut id, &mut revert) > 0
        };
        if ok {
            const NONE: u64 = 0; /* xlib::None, which is commented out for no reason */
            const POINTER_ROOT: u64 = xlib::PointerRoot as u64;
            match id {
                NONE  => Ok(None),
                POINTER_ROOT => Ok(None),
                i => Window::new(&self, i.into()).map(|w| Some(w))
            }
        } else {
            Err("XGetInputFocus() failed")
        }
    }
    pub fn atom(&self, name: &str) -> Result<Atom, &'static str> {
        let cs = try!(
            ffi::CString::new(name)
                .map_err(|_| "CString::new() failed")
        );
        let atom = unsafe {
            xlib::XInternAtom(self.xlib_display(), cs.as_ptr(), false as i32)
        };
        if atom == 0 /* xlib::None */ {
            Err("XInternAtom() failed")
        } else {
            Ok(Atom {
                id: atom,
                name: name.to_string()
            })
        }
    }
}

impl Drop for Display {
    /// Closes the connection when the `Display` is dropped
    fn drop(&mut self) {
        unsafe {
            // NOTE: XCloseDisplay() is hardcoded to return 0, so ignore it
            xlib::XCloseDisplay(self.xlib_display());
        }
    }
}

pub(super) struct Pointer {
    pub(super) pos: shapes::Point,
    pub(super) wpos: Option<shapes::Point>
}

#[derive(Clone, Eq, PartialEq)]
pub struct Atom {
    id: u64,
    pub name: String
}
