use std::convert;
use std::fmt;
use std::str;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn unpack(i: i32) -> Self {
        Color {
            r: ((i & (255 << 8 * 2)) >> 8 * 2) as u8,
            g: ((i & (255 << 8 * 1)) >> 8 * 1) as u8,
            b: ((i & (255 << 8 * 0)) >> 8 * 0) as u8
        }
    }
    pub fn pack(&self) -> i32 {
        0 |
            ((self.r as i32) << 8 * 2) |
            ((self.g as i32) << 8 * 1) |
            ((self.b as i32) << 8 * 0)
    }
}

impl str::FromStr for Color {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hex = s.to_string();
        let is = if hex.len() != 8 {
            false
        } else {
            let pre: String = hex.drain(..2).collect();
            pre == "0x".to_string()
        };
        if is {
            i32::from_str_radix(&hex, 16).map_err(|_| "not a hexadecimal color").map(|i| Color::unpack(i))
        } else {
            Err("not a hexadecimal color")
        }
    }
}

impl convert::From<i32> for Color {
    fn from(i: i32) -> Self {
        Color::unpack(i)
    }
}

impl convert::Into<i32> for Color {
    fn into(self) -> i32 {
        self.pack()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:x}", self.pack())
    }
}
