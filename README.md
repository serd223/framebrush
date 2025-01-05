![framebrush logo](assets/frame_brush.png)

[![Crate](https://img.shields.io/crates/v/framebrush.svg)](https://crates.io/crates/framebrush)
[![API](https://docs.rs/framebrush/badge.svg)](https://docs.rs/framebrush)
# framebrush
`framebrush` is a simple crate that can draw simple shapes on a frame buffer provided by the user.
 

## Why use framebrush?
### Ease of use
Scaling, drawing lines/shapes or even indexing into the frame buffer can be a bit tedious while using a simple frame buffer.

`framebrush` can handle scaling and drawing for you. And because `framebrush` doesn't have any platform specific code, (it only writes to the buffer you provided) you can use it in practically any context!

### Simplicity
`framebrush` is a fundamentally simple crate so it has a pretty simple yet generic API. All you need to do is create a `Canvas` and use its methods to draw on the buffer.

### Extendability
`framebrush` provides the `Draw` trait which lets you define your custom `draw`able shapes in addition to the primitive shapes provided by the crate.



## Hello World Example
```rs
// examples/hello.rs
use framebrush::{Canvas, RGBu32};
use minifb::{Window, WindowOptions};

const DEFAULT_WIDTH: usize = 800;
const DEFAULT_HEIGHT: usize = 600;
fn main() {
    let mut buf = vec![0; DEFAULT_WIDTH * DEFAULT_HEIGHT];

    let mut window = Window::new(
        "Hello, framebrush!",
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .unwrap();

    window.set_target_fps(144);
    while window.is_open() {
        let (width, height) = window.get_size();
        buf.resize(width * height, 0);

        // Begin drawing
        let mut canvas = Canvas::new(&mut buf, (width, height), (DEFAULT_WIDTH, DEFAULT_HEIGHT));
        canvas.fill(0);
        canvas.rect(10, 10, 30, 30, &RGBu32::Rgb(190, 96, 105));
        // End drawing
        window.update_with_buffer(&buf, width, height).unwrap();
    }
}
```
[More examples here!](https://github.com/serd223/framebrush/tree/master/examples)


## Pixel Formats
The `RGBu32::Rgb` uses the same format that [softbuffer](https://github.com/rust-windowing/softbuffer) uses, which is;

```
00000000RRRRRRRRGGGGGGGGBBBBBBBB (u32)
R: Red channel
G: Green channel
B: Blue channel
```
