use std::ptr;

use x11::xlib;

use super::Display;
use super::Window;
use super::shapes;

#[derive(Clone)]
pub struct Screen<'d> {
    d: &'d Display,
    s: ptr::NonNull<xlib::Screen>
}

impl<'d> Screen<'d> {
    /// Creates a new Screen object from the given display and screen pointer
    ///
    /// SAFETY:
    /// - s must come from the passed display
    /// - the created Screen is does not own s, s is owned by the Display
    /// - multiple instances of Screen may refer to the same screen
    pub(super) unsafe fn new_unchecked(d: &'d Display, s: ptr::NonNull<xlib::Screen>) -> Self {
        Screen { d, s }
    }

    /// Creates a shared reference to the underlying Screen object
    ///
    /// SAFETY:
    /// - no X11 functions may be called with this borrow active
    ///   as they may mutate the Screen object, which is UB while a shared
    ///   borrow is active
    /// - this borrow must not be passed to consumers of this library since
    ///   they may break these invariants
    unsafe fn get(&self) -> &xlib::Screen {
        self.s.as_ref()
    }

    pub(super) fn xlib_screen(&self) -> *mut xlib::Screen {
        self.s.as_ptr()
    }

    /// Returns the pointer coordinates relative to this screen's root window.
    ///
    /// Returns an error if the call to `XQueryPointer()` fails.
    pub fn pointer(&self) -> Result<shapes::Point, &'static str> {
        let win = self.root()?;
        let ptr = win.pointer_direct()?;
        Ok(ptr.pos)
    }

    /// Moves the pointer coordinates relative to this screen's root window.
    ///
    /// Returns an error if the call to `XQueryPointer()` fails.
    pub fn warp_pointer(&self, p: shapes::Point) -> Result<(), &'static str> {
        let win = self.root()?;
        win.warp_pointer(p)
    }

    /// Gets the root window of the screen
    ///
    /// Returns an error message if the root window does not exist.
    /// This should never happen.
    pub fn root(&self) -> Result<Window<'d>, &'static str> {
        // SAFETY: borrow does not overlap with an X11 call
        Window::new(self.d, unsafe { self.get() }.root.into())
    }

    /// Gets the width of the screen in pixels
    pub fn width(&self) -> u32 {
        // SAFETY: borrow does not overlap with an X11 call
        unsafe { self.get() }.width as u32
    }

    /// Gets the height of the screen in pixels
    pub fn height(&self) -> u32 {
        // SAFETY: borrow does not overlap with an X11 call
        unsafe { self.get() }.height as u32
    }

    pub fn rectangle(&self) -> shapes::PositionedRectangle {
        shapes::PositionedRectangle::new(0, 0, self.width(), self.height())
    }
}
