extern crate wtools;

fn run() -> wtools::OrErrorStr<()> {
    let disp = try!(wtools::Display::open());
    let scrn = try!(disp.screen());
    println!("Got {}x{} Screen with root = 0x{:x}", scrn.width(), scrn.height(), scrn.root().id());
    Ok(())
}

fn main() {
   wtools::handle_error(run());
}
