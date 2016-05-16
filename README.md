# wtools - Coreutils for Xorg, implemented in Rust

This project aims to implement simple tools for window management in Xorg.
wtools is inspired by the [wmutils](https://github.com/wmutils/core) project.

This project uses Xlib (Cargo crate [x11](https://crates.io/crates/x11)) to
communicate with Xorg, as XCB was very hard to abstract to my needs.

## Building

This project is inteded to be build with Cargo, the Rust package manager.

This project specifically requires (as of 2016.05.08) a nightly build of rustc
to compile, as it uses some macro code (`parse_args!` inexplicably doesn't
work on release rustc) and features (`pub_restricted`) that don't work with
release rustc.

## Documentation

Documentation is managed via rustdoc, and specifically, Cargo's implementation
of it, `cargo doc`.

## Contributing

Contributions are welcome, although I might not accept all pull requests,
simply as a matter of taste. Don't be afraid to create a fork. This repo is not
intended to become the "master" repo where everybody pulls from if this project
is ever used by people other than me.

## License

This project is licensed under the GNU GPLv3 and later licenses. The GNU GPLv3 is
provided in the LICENSE file.

## Current tools

All tools have a `--help` switch that gives more info about their usage. Long
options are available, but omitted here for brevity.

### wborder

`wborder [-c color] [-s size] wid`

Change a window's border color and width.

### winfo

`winfo [-xywhbimo] wid`

Gives information about a window.

### wlist

`wlist [-uoa] [wid]`

Lists the children of wid, defaults to the root window.

### wmove

`wmove [-ra] x y wid`

Move a window relatively or absolutely.

### wresize

`wresize [-ra] x y wid`

Resize a window relatively or absolutely.

### wwindow

`wwindow [-rf]`

Returns the window id of special windows.

Currently supports the root window and the focused window. Prints an error if
no window is focused in focus mode rather than printing the reserved resource
ids for None and PointerRoot.
