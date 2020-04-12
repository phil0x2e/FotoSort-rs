# FotoSort-rs
A simple command line tool for reviewing images and sorting them into different folders.

[FotSort](https://github.com/phil0x2e/FotoSort) but using rust and [image\_window](https://github.com/phil0x2e/image_window).

Install with `cargo install --path .`

After that you can, if *.cargo/bin* is in your PATH just call it with `fotosort ..`

If you don't want to install it you have compile it with --build flag, or it will be unsusably slow.

For help run `fotosort --help`.

## Usage
- Left/Right: Previous/Next Window
- 1..5: Copy (default) or move (when -m is set) current image to folder fs[1..5]
- C+1..5: Copy current image to folder fs[1..5]
- M+1..5: Move current image to folder fs[1..5]
- R / Shift+R: Rotate preview 90° clockwise / counter clockwise (rotation is not saved)
- Del: Delete current image (confirm with Y)
- H: Display this usage information
- F5: reload current image
- Esc: Quit

## TODO
- Named directories instead of numbers
- Not only accept files as arguments, but also directories.

