# wtools - Coreutils for Xorg, implemented in Rust

This project aims to implement simple tools for window management in Xorg.
wtools is inspired by the [wmutils](https://github.com/wmutils/core) project.

This project uses Xlib (Cargo crate [x11](https://crates.io/crates/x11)) to
communicate with Xorg, as XCB was very hard to abstract to my needs.

# Current tools

## wmove

`wmove [-r -a] x y wid`

Moves a window on the XServer relatively or absolutely.

## wresize

`wresize [-r -a] x y wid`

Resizes a window on the XServer relatively or absolutely.
