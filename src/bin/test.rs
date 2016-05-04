extern crate wtools;

fn main() {
    let disp = wtools::Display::open();
    println!("{}", match disp {
        Ok(_) => "YaY, it works!".to_string(),
        Err(s) => format!("Doesn't work: {}", s)
    });
}
