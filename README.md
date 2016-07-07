# ellsworth

A small program to generate an image in the style of
[Ellsworth Kelly](https://en.wikipedia.org/wiki/Ellsworth_Kelly).

## Getting started

Requires [Rust](https://rust-lang.org).

Pull down the project:

    git clone git@github.com:mxhold/ellsworth.git
    cd ellsworth

Add a file `colors.csv` with the following format:

    name,red,green,blue
    red,255,0,0
    green,0,255,0
    blue,0,0,255

Then to generate an image at `image.png` 500x500 pixels with 10x10 tiles:

    cargo build --release
    ./target/release/ellsworth 500 10

Example image:

![Image with large multicolored squares](image.png)
