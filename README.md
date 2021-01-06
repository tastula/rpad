# RPad

RPad is a small tool for replacing uneven padding with unified one. It works by
cutting away the whole monochromic area around given image and embedding the
resulting image to the center of a new one. Like this!

| Before             | After            |
| ------------------ | ---------------- |
| ![uneven logo][b1] | ![even logo][a1] |

[b1]: res/ff1.png
[a1]: res/ff2.png

## Building & installation

When [Rust and Cargo](https://rustup.rs) are installed, just run `cargo
install`.

## Usage

```
rpad <input> [output] [size]

input   (required) path to input image
output  (optional) path to output directory, default ~
size    (optional) padding size in pixels, default 30
```
