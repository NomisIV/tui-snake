# It's that snake game...

...but written as a TUI program in rust.

## About

This program uses [termion](https://crates.io/crates/termion) for generating escape codes for the graphics.
The only other dependency is [rand](https://crates.io/crates/rand) for randomizing the position of the food piece.

The implementation is multithreaded:
- One thread for getting the inputs (waiting for input is blocking)...
- ...another thread for the ticks (thread::sleep is also quite blocking)...
- ...and lastly a main thread responsible for recieving input from both threads.

Communication between the threads is done by using [mpsc channels](https://doc.rust-lang.org/stable/std/sync/mpsc/),
which I'd never heard of before starting this project,
but they proved to be incredibly useful.

## Installing

`cargo install tui-snake`

## Running

`tui-snake`

Currently the program doesn't support any command line options / flags,
and the entire terminal is used as a playing field.
This might come to change with later versions.

To exit the program while running,
press `Esc` or `Ctrl+c`.
