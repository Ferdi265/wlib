extern crate wtools;

fn run() -> wtools::OrErrorStr<()> {
    let disp = try!(wtools::Display::open());
    let scrn = try!(disp.screen());
    println!("Got {}x{} Screen with root = 0x{:x}", scrn.width(), scrn.height(), scrn.root().id());
    let win = disp.window(0x00e00009);
    win.position(10, 10);
    Ok(())
}

fn main() {
   wtools::handle_error(run());
}
